// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::cli_types::{CommandAction, WalletContextOptions};
use async_trait::async_trait;
use clap::Parser;
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{StructTag, TypeTag},
};
use moveos_types::transaction::MoveAction;
use rooch_types::error::RoochResult;
use serde_reflection::{Samples, Tracer, TracerConfig};
use std::fmt::Debug;
use std::fs;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct ExportRoochTypesCommand {
    #[clap(flatten)]
    pub context_options: WalletContextOptions,

    #[clap(long, default_value = "./target/rooch_types.yaml")]
    pub file_path: String,
}

#[async_trait]
impl CommandAction<()> for ExportRoochTypesCommand {
    async fn execute(self) -> RoochResult<()> {
        export_rooch_types_yaml(&self.file_path)
    }
}

use serde_yaml::Value;

fn convert_enum(yaml_value: &mut Value) {
    match yaml_value {
        Value::Mapping(map) => {
            for (k, v) in map.iter_mut() {
                if k.as_str() == Some("ENUM") {
                    if let Value::Mapping(enum_map) = v {
                        let mut new_enum_map = serde_yaml::Mapping::new();
                        for (k, v) in enum_map.iter() {
                            if let Ok(num) = k.as_str().unwrap().parse::<i32>() {
                                new_enum_map.insert(Value::Number(num.into()), v.clone());
                            }
                        }
                        *enum_map = new_enum_map;
                    }
                } else {
                    convert_enum(v);
                }
            }
        }
        Value::Sequence(seq) => {
            for v in seq {
                convert_enum(v);
            }
        }
        _ => {}
    }
}

fn export_rooch_types_yaml(file_path: &String) -> RoochResult<()> {
    let mut tracer = Tracer::new(TracerConfig::default());

    // Predefine StructTag to prevent recursive definitions from reporting errors.
    let mut samples = Samples::new();
    let example_struct_tag = StructTag {
        address: AccountAddress::random(),
        module: Identifier::new("Module").unwrap(),
        name: Identifier::new("Name").unwrap(),
        type_params: vec![TypeTag::Bool],
    };
    tracer
        .trace_value(&mut samples, &example_struct_tag)
        .unwrap();

    let example_type_tag = TypeTag::Struct(Box::new(example_struct_tag));
    tracer.trace_value(&mut samples, &example_type_tag).unwrap();

    // Define TypeTag and MoveAction
    tracer.trace_type::<TypeTag>(&samples).unwrap();
    tracer.trace_type::<MoveAction>(&samples).unwrap();

    match tracer.registry() {
        Ok(registry) => {
            let data: String = serde_json::to_string_pretty(&registry).unwrap();

            // Since serde_yaml does not support nested enumerations, the registry is first converted to json and then converted to yaml.
            let json_value: serde_json::Value = serde_json::from_str(data.as_str()).unwrap();

            // Change json_value to yaml
            let yaml_string = serde_yaml::to_string(&json_value).unwrap();

            // Replace AccountAddress.NEWTYPESTRUCT.TUPLEARRAY.SIZE to 32
            let mut yaml_value: serde_yaml::Value =
                serde_yaml::from_str(yaml_string.as_str()).unwrap();
            yaml_value["AccountAddress"]["NEWTYPESTRUCT"]["TUPLEARRAY"]["SIZE"] =
                serde_yaml::Value::from(32);

            // Convert ENUM key from string to number
            convert_enum(&mut yaml_value);

            let replaced_yaml_string = serde_yaml::to_string(&yaml_value).unwrap();

            let path = Path::new(file_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?; // 创建所有父目录
            }

            fs::write(path, replaced_yaml_string)?; // 创建文件并写入数据

            println!("export rooch types to file: {file_path} ok!");
        }
        Err(e) => {
            let msg = e.explanation();
            println!("export rooch_types.yaml error: {msg}");
        }
    }

    Ok(())
}

// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::cli_types::{CommandAction, WalletContextOptions};
use async_trait::async_trait;
use clap::Parser;
use rooch_types::error::RoochResult;

use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "public/dashboard/"]
struct Asset;

#[get("/")]
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

#[get("/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

/// Start Rooch Dashboard
#[derive(Parser)]
pub struct Dashboard {
    /// Accept defaults config, default true
    #[clap(short = 'y', long = "yes", default_value_t = true)]
    pub accept_defaults: bool,
    #[clap(flatten)]
    pub context_options: WalletContextOptions,
}

#[async_trait]
impl CommandAction<String> for Dashboard {
    async fn execute(self) -> RoochResult<String> {
        let s = rocket::build().mount("/", routes![index, dist]);

        let _ = s.launch().await;

        Ok("Rocket: deorbit.".to_owned())
    }
}

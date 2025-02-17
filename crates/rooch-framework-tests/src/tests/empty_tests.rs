// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::binding_test;
use moveos_types::tx_context::TxContext;

#[test]
fn test_empty() {
    let binding_test = binding_test::RustBindingTest::new().unwrap();
    let empty = binding_test.as_module_bundle::<rooch_framework::bindings::empty::Empty>();
    let ctx = TxContext::random_for_testing_only();
    empty.empty(&ctx).unwrap();
}

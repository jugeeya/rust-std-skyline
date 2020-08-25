#![warn(clippy::double_parens)]
#![allow(dead_code)]
#![feature(custom_inner_attributes)]
#![rustfmt::skip]

fn dummy_fn<T>(_: T) {}

struct DummyStruct;

impl DummyStruct {
    fn dummy_method<T>(self, _: T) {}
}

fn simple_double_parens() -> i32 {
    ((0))
}

fn fn_double_parens() {
    dummy_fn((0));
}

fn method_double_parens(x: DummyStruct) {
    x.dummy_method((0));
}

fn tuple_double_parens() -> (i32, i32) {
    ((1, 2))
}

fn unit_double_parens() {
    (())
}

fn fn_tuple_ok() {
    dummy_fn((1, 2));
}

fn method_tuple_ok(x: DummyStruct) {
    x.dummy_method((1, 2));
}

fn fn_unit_ok() {
    dummy_fn(());
}

fn method_unit_ok(x: DummyStruct) {
    x.dummy_method(());
}

// Issue #3206
fn inside_macro() {
    assert_eq!((1, 2), (1, 2), "Error");
    assert_eq!(((1, 2)), (1, 2), "Error");
}

fn main() {}

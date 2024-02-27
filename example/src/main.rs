use proc_macro_test::{StructName1, StructName2};

#[derive(StructName1)]
struct Yeet;

#[derive(StructName2)]
struct Yote;

fn main() {
    let yeet = Yeet;
    let yote = Yote;

    println!("First: {}, Second: {};", yeet.struct_name(), yote.struct_name());
}

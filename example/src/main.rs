use proc_macro_test::{Create, hide};

#[hide]
struct Example();

#[derive(Create)]
#[hide(off)]
struct Coordinate {
    x: i32,
    y: i32
}

fn main() {
    let coord = create();

    println!("Coordinate ({}, {})", coord.x, coord.y);
}

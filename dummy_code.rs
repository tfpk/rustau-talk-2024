// Define a macro...
proc_macro Addable(code: Code) -> Code {
    println!("Running addable...")
    let struct_name = code.struct_name;

    extra_code = create_code(...);

    extra_code
}

#[derive(Addable)]
struct Coordinate {
    x: i32,
    y: i32
}

////// DURING COMPILATION:

impl Coordinate {
    fn add(&self, other: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

//////

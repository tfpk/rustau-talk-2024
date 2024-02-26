// Define a macro...
proc_macro procedural_macro(code: Code) -> Code {
    code.convert_to_assignment()

}

// Write this code...
fn func_before_macro() -> i32 {
    let mut a = 1;
    procedural_macro!(set a as 2);
    return a
}

// This is what really happens
fn func_after_macro() -> i32 {
    let mut a = 1;
    a = 2;
    return a
}

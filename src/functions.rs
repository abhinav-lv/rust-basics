pub fn functions() {
    function_pointer();
    function_as_parameter(add);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn function_pointer() {
    type BlehType = fn(i32, i32) -> i32;
    let add_alias: BlehType = add;
    dbg!(add_alias(56, 69));
}

fn function_as_parameter(lol: fn(i32, i32) -> i32) {
    dbg!(lol(69,420));
}

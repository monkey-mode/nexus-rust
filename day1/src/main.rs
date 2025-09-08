fn main() {
    println!("Hello, world!");
    variable_println_assert_eq()
}

fn variable_println_assert_eq(){
    // Define immutable variable.
    let count = 0;

    // {} mean param_0.
    println!("1. count = {}", count);

    // Define mutable variable.
    let mut count = 1;
    count += 1;

    println!("2. {0} = {1:#?}", "count", count);

    if count == 2{
        println!("3. count = {count}");
    }

    assert_eq!(count,2);

    println!("4. count = {count} = 0x{count:x}");
    println!("4. count = {count} = {count:b}")
}

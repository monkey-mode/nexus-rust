fn main() {
    println!("----------variable_println_assert_eq----------");
    variable_println_assert_eq();

    println!("----------for_while_loop_break----------");
    for_while_loop_break();

    println!("----------fn_const_static_return_format----------");
    fn_const_static_return_format();
}

fn variable_println_assert_eq(){
    // Define immutable variable.
    let count = 0;

    // {} mean param_0.
    println!("1. count = {}", count);

    // Define mutable variable.
    let mut count = 1;

    // So we can change it.
    count += 1;

    // {0} mean param_0.
    // {1} mean param_1.
    //            ╭────────────────╮
    println!("2. {0} = {1:#?}", "count", count);
    //                  ╰──────────────────╯
    // # mean pretty print.
    // ? mean debug.

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("3. count = {count}");
    }

    // Assert that count is equal 2.
    assert_eq!(count, 2);

    // As base 16 hexadecimal by adding    👇.
    println!("4. count = {count} = 0x{count:x}");
    println!("4. count = {count} = {count:b}");
}

fn for_while_loop_break(){
    //  👇 Mutable so we change the value later.
    let mut count = 0;

    // This .. 👇 mean range i from 0 to 7.
    for _i in 0..8 { // _i mean we won't use i
        count += 1;
    }

    println!("1. count = {count}");

    // This .. 👇 mean range i from 0 to 8.
    for i in 0..=8 {
        count += i;
    }

    println!("2. count = {count}", count = count);

    // 👇 This is how we loop element (e).
    for e in ["a","b","c"] {
        println!("3. {e}");
    }

    //  👇 This is index (i) can be use by 👇 call enumerate fn.
    for (i, e) in ["a","b","c"].iter().enumerate() {
        println!("4. {i} = {e}");
    }

    // while
    while count < 50 {
        count += 1;
    }

    println!("5. count = {0}", count);

    // loop
    loop {
        count += 1;
        if count >= 100 {
            break;
        }
    }

    println!("6. count = {}", count);

    // loop and break
    'outer: loop {
        count += 1;

        // Break at 200
        if count >= 200 {
            // Never reach here because 👇.
            break;
        } else {
            // Inner loop
            loop {
                count += 1;
                // Because this will break first.
                if count >= 150 {
                    break 'outer;
                }
            }
        }
    }

    println!("7. count = {}", count);
}

const COUNT: &str = "count";
static TOTAL: u32 = 0;

fn add(a: i32, b: i32) -> i32{
    a+b
}

fn fn_const_static_return_format(){
    assert!(add(1,2)==3);

    let result = format!("{COUNT} = {}", add(1,9));
    println!("1. {result}");

    // unsafe{
    //     total = add(3,4) as u32;

    //     assert_eq!(total,7);
    // }
    println!("{TOTAL}");

}
fn main() {
    println!("----------variable_println_assert_eq----------");
    variable_println_assert_eq();

    println!("----------for_while_loop_break----------");
    for_while_loop_break();
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
    let mut count = 0;

    for _i in 0..8{
        count += 1;
    }

    println!("1. count = {count}");


    for i in 0..8 {
        count += i
    }

    println!("2. count = {count}",count = count);

    for e in ["a","b","c"]{
        println!("3. {e}");
    }


    for (i,e) in ["a","b","c"].iter().enumerate(){
        println!("4. {i} = {e}");
    }

    while count  < 50{
        count += 1;
    }

    println!("5. count = {0}", count);

    loop {
        count += 1;
        if count >= 100{
            break;
        }
    }

    println!("6. count = {}",count);

    'outer: loop{
        count += 1;
        if count >= 200{
            break;
        }else{
            loop {
                count += 1;
                if count >= 150{
                    break 'outer;
                }
            }
        }
    }

    println!("7. count = {}",count);
}
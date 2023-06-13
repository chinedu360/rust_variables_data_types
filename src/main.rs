const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let mut y = 1;
    println!("{y}");
    y += 1;
    println!("{y} {THREE_HOURS_IN_SECONDS}");

    shadowing();
    floating_number();
    boolean();
    char_type();
    tuple();
    array();
}

fn shadowing() {
    let x: i32 = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x in the outer scope is: {x}")
}

fn floating_number() {
    let x = 2.3;
    let y: f32 = 3.0;

    println!("This is floating:{x} {y}");
}

fn boolean() {
    let t = true;
    let f = false;

    println!("This is boolean: {t} {f}")
}

fn char_type() {
    let n = 'c';
    let emoji = '🥰';

    println!("The char: {n} {emoji}")
}

fn tuple() {
    let tup: (i32, f64, u8, char, bool) = (500, 5.4, 1, 'c', false);

    let (a, b, c, d, e) = tup;

    println!("The tuple: {a} {b} {c} {d} {e}");
    
    let third_index = tup.3;
    println!("Third Index: {third_index}");
}

fn array() {
    let _a = [1, 2, 3, 4, 5, 6, 7];
    let months = ["january", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let first = months[0];
    let second = months[1];
    println!(" {first} {second} ");
    let _a = [3; 7];
}
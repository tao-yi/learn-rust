const MAX_POINTS: u32 = 100_000;

fn main() {
    // variables()
    // variable_type();
    // operation();
    // tuple();
    // another_function(5, 1);

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("{}", y);

    // let c = hello();
    // println!("{}", c);

    // let num = 3;

    // if num < 5 {
    //     println!("true");
    // } else {
    //     println!("false");
    // }

    // let d = if num < 5 { "bigger" } else { "smaller" };
    // println!("{}", d);

    // repetition();

    // loop_value()
    // for_loop();
    range_loop();
}

fn variables() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    // shadowing, declare a new variable x
    // previous x is not available anymore
    let x = x + 1;
    let mut x = x * 2;
    x *= 2;
    println!("{}", x);

    let spaces = "    "; // &str
    let spaces = spaces.len(); // usize
    println!("{}", spaces);
}

fn variable_type() {
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    let c = 57u8;

    let c: u8 = 57;

    let n = 32;

    let f = 32.123;
    let f = 32.1;

    let mut i: u8 = 255;
}

fn operation() {
    // addition
    let sum = 5 + 10; // i32

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 10.1, 15);

    let (x, y, z) = tup;

    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{}, {}, {}", x, y, z);

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
    println!("{}", arr[3]);
    println!("{}", arr[4]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let index = [12, 13, 14, 15];
    let month = months[index[0]];
    println!("{}", month);
}

fn another_function(x: i32, y: i64) {
    println!("{}", x);
    println!("{}", y);
}

fn hello() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn repetition() {
    let mut i = 1;
    'outer: loop {
        i += 1;
        println!("{}", i);
        'inner: loop {
            i -= 2;
            if i == -5 {
                continue 'outer;
            }
            println!("{}", i);
            if i == -10 {
                break 'outer;
            }
        }
    }
    println!("{}", i);
}

fn loop_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }
}

fn range_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    // std::ops::Range<i32>
    let c = (1..4);
    for n in c {
        println!("{}!", n);
    }
}

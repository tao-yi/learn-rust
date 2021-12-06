use std::io; // prelude
use rand::Rng; // trait 类似于其他语言的 interface
use std::cmp::Ordering;

fn main() {
    println!("猜一个数");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);


    loop {
        println!("Please input your guess.");

        // 所有的变量默认是不可变的
        // let foo = 1;
        // let bar = foo;
        // cannot assign twice to immutable variable `foo`
        // foo = 2;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        println!("你猜测的数是: {}", guess);

        // shadow
        // let guess:u32 =  guess.trim().parse().expect("Please type a number");
        let guess:u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字");
                continue
            }
        };

        // 和u32进行比较，会修改secret_number的类型
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!") ;
                break;
            }
        }
    }
}

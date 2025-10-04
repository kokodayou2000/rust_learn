use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // i32 能自动变成 u32
    let secret_num = rand::rng().random_range(1..=100);
    println!("The secret number is {}.", secret_num);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        // 表达式和语句，如果没有 ; 就是表达式，能被返回，如果携带了 ; 就是语句，无法被返回
        // 所以 Ok(num) => { num } num 不能携带 ;
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => {
                num
            },
            Err(err) => {
                // 如果输入数据非字符串 不报错，进行下一次循环
                println!("Error: {}", err);
                continue
            },
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => {
                println!("less.");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("greater.");
            }
        }
        println!("You guessed: {}", guess);
    }
    println!("end");
}

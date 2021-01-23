use std::io; // 输入输出库引入当前作用域（来自标准库std）
use std::cmp::Ordering; //
use rand::Rng; // 引入random crate来实现随机数功能

fn main() {

    // First step: Get a guessing number from keyboard and print it

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101); // generate a random number from 0-100

    //println!("The secret number is {}",secret_number);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        // Second step: Compare two numbers

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}

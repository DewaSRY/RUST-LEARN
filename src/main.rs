use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Guess the number !");
        let secret_number=rand::thread_rng().gen_range(1..=10);
        println!("the secret number is {secret_number} ");
        println!("please input your guess");
        let mut guess=String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("filed to read line");
        let guess : u32=match guess.trim().parse() {
            Ok(num) => num,
            Err(err) =>{
                println!("please type a number, we found error with massage\n{err}");
                continue;
            },
        }; 

        println!("you guessed :{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less=>println!("too small!"),
            Ordering::Greater=>println!("Too Big!"),
            Ordering::Equal=>{
                println!("you right");
                break;
            }
        }
    }

    println!("Hello, world!");
}

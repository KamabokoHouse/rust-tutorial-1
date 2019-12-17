extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("数字当てゲーム");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("数字を入力してください");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("読み取りに失敗しました");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("予想値: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}

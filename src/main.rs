use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let mut guess = "42".parse().expect("111");
}

fn main1() {
    println!("========> 猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{}", secret_number);

    loop {
        println!("请输入猜测是数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess:u32 = guess.trim().parse().expect("请输入一个整数!");
        println!("你猜测的数字是{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
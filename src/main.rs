use std::cmp::Ordering;
use std::io;
use std::net::IpAddr;
use rand::Rng;

// fn main1() {
//     println!("========> 猜数游戏");
//     let secret_number = rand::thread_rng().gen_range(1, 101);
//     println!("神秘数字是{}", secret_number);
//
//     loop {
//         println!("请输入猜测是数字");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读取行");
//         let guess:u32 = guess.trim().parse().expect("请输入一个整数!");
//         println!("你猜测的数字是{}", guess);
//
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("to small"),
//             Ordering::Greater => println!("to big"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

// fn main2() {
//     let mut guess: u32 = "42".parse().expect("111");
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
// fn main() {
//     let email = String::from("someone@example.com");
//     println!("{}", email);
//     let user = User{
//          email,
//          username: String::from("someone"),
//          active: true,
//          sign_in_count: 1
//     };
//     // println!("{}", email); --> email已经失效, 发生了移动 --> user持有了email值
//
//     let user2 = User{
//         username: String::from("someone2"),
//         active: false,
//         ..user
//     };
//     println!("{}", user2.email);
//     println!("{}", user.sign_in_count);
//
// }

// #[derive(Debug)] // 导入调试库 #[derive(Debug)], 之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle { // struct的方法在impl块中定义
//     fn area(&self) -> u32{ // 第一个参数是self, 表示方法被调用的struct的实例
//         return self.width * self.height;
//     }
// }
//
//
// fn main() {
//     let rect = Rectangle {
//         width: 10,
//         height: 10,
//     };
//     println!("area = {}", rect.area());
//     println!("{:#?}", rect);
//
// }

#[derive(Debug)]
enum IpAddrKind {
    V4([u8; 4]),
    V6([u8; 16]),
}

impl IpAddrKind {
    fn value_in_ipAddrKind(&self) -> u8 {
        match self {
            IpAddrKind::V4(arr) => {
                println!("{:?}", arr);
                arr[0]
            },
            IpAddrKind::V6(arr) => {
                println!("{:?}", arr);
                return arr[0];
            }
        }
    }
}

fn main() {
    let v4 = IpAddrKind::V4([1, 1, 1, 1]);
    let v6 = IpAddrKind::V6;
    println!("{:#?}", v4);
    println!("{:#?}", v6);
}

// fn main() {
//
// }
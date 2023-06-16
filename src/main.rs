use std::env;
use std::process;

use rust_learn::Config;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("error, {}", err);

//         process::exit(1)
//     });

//     if let Err(e) = rust_learn::run(config) {
//         eprintln!("error, {}", e);
//         process::exit(1)
//     }

//     // println!("config, {:?}", config)
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// enum ShirtColor {
//     Red,
//     Blue
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1
//             }
//         }

//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }


use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("spawned thredd {}", i);

    //         thread::sleep(Duration::from_millis(1000))
    //     }
    // });

    // for i in 1..5 {
    //     println!("main thread {}", i);

    //     thread::sleep(Duration::from_millis(1000))
    // }

    // handle.join().unwrap();

    // thread::spawn(|| {
    //     for i in 11..13 {
    //         println!("spawned another thredd {}", i);

    //         thread::sleep(Duration::from_millis(1000))
    //     }
    // }).join().unwrap();

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here are {:?}", v);
    // });

    // handle.join().unwrap();

    let ( tx, rx ) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();

        // println!("{val}");
    });

    let received = rx.recv().unwrap();

    println!("got: {}", received);


}

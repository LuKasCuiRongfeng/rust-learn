use rand::Rng;
use std::{cmp::Ordering, io, process::Command};

fn main() {
    let mut x = 10;

    x = 3;

    const FUCK: i64 = 12 * 45;

    let y = 0xff;

    let z = 4.5;

    let xx = true;

    let ch = 'f';

    let tup = ('f', 23, true);

    let (a, b, c) = tup;

    let arr = [23, 54, 324];

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);

    // println!("your secret_numb is: {secret_number}");

    loop {
        println!("Please input your gusess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };

        println!("your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }

    ant_fuck(23);

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

/** fuck */
fn ant_fuck(x: i32) {
    println!("anot func, {x}");
}

fn five() -> i64 {
    6;

    if 2 < 4 {
        println!("fuck")
    }

    let a = if true { 3 } else { 5 };

    'label: loop {
        println!("432532");

        loop {
            break 'label;
        }
    }

    for e in 34..345 {
        println!("{e}");
    }

    let mut s = String::from("hello");

    s.push_str("string");

    calc_len(&mut s);

    let s2 = s;

    let s3 = "fdfdfdf";

    return 34;
}

fn calc_len(s: &mut String) -> usize {
    s.push_str("fuck your mother");
    let mut user = User {
        name: String::from("crf"),
        age: 23,
        email: String::from("dfdf"),
    };

    user.age = 4;

    println!("{:#?}", user);

    let mut i = IP::V6(String::from("fuck"));

    i = IP::V4(String::from("do"));

    i.call();

    let some = Option::Some(43);

    let some1: Option<i32> = Some(12);

    return s.len();
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

impl User {
    fn test(&self) -> &String {
        &self.name
    }
}

struct Vector2(i32, i32);

enum IP {
    V4(String),
    V6(String)
}

impl IP {
    fn call(&self) {

    }
}

fn testIP(ip: IP) -> i32 {
    match ip {
        IP::V4(_) => 5,
        IP::V6(_) => 2
    }
}
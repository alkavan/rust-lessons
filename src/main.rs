mod behavior;
mod collections;
mod colors;
mod composition;

use crate::collections::{hash_map, hash_set, queue, vector};
use crate::composition::rpg;
use behavior::pets;
use colors::colors;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    hello_world();
    // guess_a_number();

    // variables();
    // constants();
    // tuple();

    // colors();
    // pets();

    // vector();
    // queue();
    // hash_set();
    // hash_map();

    // rpg();
}

fn hello_world() {
    println!("Hello, world!");
}

fn guess_a_number() {
    println!("Welcome to my awesome Guess The Number game!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();

    println!("Please input your guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);

    let guess_int: u32 = guess.trim().parse().expect("Please type a number!");
    let secret_number = secret_number as u32;

    match guess_int.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number was: {}", secret_number);
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

const U8_MAX: u8 = 255;

fn constants() {
    let max_u8 = max_u8();
    println!("Array: {:?}", max_u8);
}

const fn max_u8() -> [u8; 3] {
    let i8_max: i8 = i8::MAX;
    // return &number; // copy

    // return [U8_MAX, i8_max];
    return [U8_MAX, i8_max as u8, U8_MAX];
}

pub fn tuple() {
    let point = (34, 144, 233);
    let (x, y, z) = point;
    println!("({}, {}, {})", x, y, z);
    println!("({}, {}, {})", point.0, point.1, point.2 );
}

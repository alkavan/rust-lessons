use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::io;

fn main() {
    hello_world();
    // guess_a_number();
    variables();
    constants();
    colors();
}

fn hello_world() {
    println!("Hello, world!");
}

fn guess_a_number() {
    println!("Welcome to my awesome Guess The Number game!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();
    println!("Please input your guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let mut guess_int: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);

    let secret_number = secret_number as u32;

    match guess_int.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number was: {}", secret_number);
}

fn variables() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    println!("The value of x is: {}", x);
}

const U8_MAX: u8 = 255;

fn constants() {
    let array = [U8_MAX; 5]; // constant is literally alias to another static value
    println!("Array: {:?}", array);
    let foo = max_u8();
    println!("U8_MAX: {}", max_u8());
}

const fn max_u8() -> &'static u8 {
    let number: u8 = 128;
    // return &number; // copy
    return &U8_MAX;
}

type Color = (i32, i32, i32);

#[derive(Debug)]
struct KewlColor(Color);

fn colors() {
    const COLOR_BLUE: (i32, i32, i32) = (0, 0, 255); // rgb
    println!("Blue (constant): {:?}", COLOR_BLUE);

    let blue = KewlColor((0, 0, 255));
    println!("Blue1: {}", blue);

    let blue2 = Color2 { r: 0, g: 0, b: 255 };
    println!("Blue2: {:?}", blue2);

    let blue2 = Color2::new(0, 0, 255);
    println!("Blue3: {:?}", blue2);
}

impl fmt::Display for KewlColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Color2 {
    r: u8,
    g: u8,
    b: u8,
}

impl Color2 {
    fn new(r: u8, g: u8, b: u8) -> Color2 {
        return Color2 { r, g, b };
    }
}

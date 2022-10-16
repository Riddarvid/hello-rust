use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    //run_guessing_game(100);
    ownership_practice();
}

fn run_guessing_game(upper_limit: u32) {
    println!("The number is between 1 and {upper_limit}. Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=upper_limit);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess, please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn ownership_practice() {
    let mut s = String::from("hello");
    let mut t = "hello";
    t = "hejdåååååå";
    s.push_str("svejs");
    println!("{}", s);

    //Size of 5 is known at compile time, therefore 5 lives on the stack
    let x = 5;
    //Make a copy of x and bind it to y
    let y = x;

    //String content is stored on the heap, pointer stored on the stack
    let s1 = String::from("hello");
    //When copying s1 from the stack, we copy the pointer that refers to the exact same
    //memory area as s1 does. That is, s1 and s2 now refer to the exact same string object.
    let s2 = s1;
    //The following line is not valid, since ownership of the memory area used by the string has
    //transferred over to s2.
    //println!("{}", s1);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}, s2 = {s4}");
}
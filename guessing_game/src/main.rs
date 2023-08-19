use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    let mut guess_no: i32;
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess_no = guess.trim().parse().expect("Please type a number!");
        if guess_no > secret_number {
            println!("Too big!");
        } else if guess_no < secret_number {
            println!("Too small!");
        } else {
            println!("You win!");
            break;
        }
    }
}

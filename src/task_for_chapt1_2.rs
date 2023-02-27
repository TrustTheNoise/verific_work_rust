 use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn task_for_chapt1_2() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut num_of_tries = 0;
    let _pohui: i32 = -5;
    loop{
        println!("\n\nВведите число: ");
        let mut guess = String:: new();

        io::stdin()
        .read_line(&mut guess)
        .expect("ALARM!!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        num_of_tries += 1;

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {println!("You Win!\nYou have {num_of_tries} tries"); break;}
            Ordering::Greater => println!("Too big!")
        }
    }
}

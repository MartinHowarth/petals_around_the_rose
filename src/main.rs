extern crate rand;
#[macro_use(c)]
extern crate cute;

use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to petals around the rose!");

    loop {
        play_game();
    }
}

fn play_game() {
    println!("Enter the number of dice to roll:");
    let mut num_rolls = String::new();

    io::stdin().read_line(&mut num_rolls)
        .expect("Failed to read line");

    let num_rolls: i32 = match num_rolls.trim().parse() {
        Ok(num) => num,
        Err(_)  => return,
    };

    let dice_rolls = c![roll_dice(), for _x in 0..num_rolls];

    println!("Rolled dice are: {:?}", dice_rolls);

    let num_petals = count_petals(dice_rolls);

    println!("Number of petals is: {}", num_petals);

}

fn roll_dice() -> i32 {
    return rand::thread_rng().gen_range(1, 7);
}

fn count_petals(dice: Vec<i32>) -> i32 {
    let mut counter = 0;
    for die in dice {
        match die {
            3 => {
                counter = counter + 2;
            },
            5 => {
                counter = counter + 4;
            },
            _ => {;},
        };
    }
    return counter;
}
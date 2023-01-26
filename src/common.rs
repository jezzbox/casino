use std::{io};

pub fn read_user_input() -> String {
    let mut user_input  = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input = user_input.trim();
    user_input.to_string()
}

pub fn read_int() -> u32 {
    let user_input = read_user_input();
    let num_players: u32 = user_input.trim().parse().expect("not a number");
    num_players
}

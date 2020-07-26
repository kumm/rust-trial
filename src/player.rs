use crate::table::*;
use std::io;


/*
struct CliPlayer;

impl CliPlayer {
    fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().to_string()
    }

    fn read_target() -> (usize, usize) {
        let input = get_input("your turn");
        if input.len() == 2 {
            for char in input {
                match char {
                    col @ 'A' ... 'J' => 
                    row @ '0' ... '9' =>
                    _ =>
                }
            }
        }
        (0,0)
    }
}

impl Player for CliPlayer {
    fn step(&table: Table, figure: Figure) -> (usize, usize) {

    }

}
*/

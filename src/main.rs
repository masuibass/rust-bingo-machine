use std::io::{self, Write};

use rand::prelude::*;

#[derive(Debug)]
struct BingoMachine {
    available_balls: Vec<u8>,
    drawn_balls: Vec<u8>,
}

impl BingoMachine {
    fn new() -> Self {
        Self {
            available_balls: (1..=75).collect(),
            drawn_balls: Vec::new(),
        }
    }

    fn available_balls(&self) -> &Vec<u8> {
        &self.available_balls
    }

    fn drawn_balls(&self) -> &Vec<u8> {
        &self.drawn_balls
    }

    fn draw_ball(&mut self) -> Result<u8, &str> {
        match self
            .available_balls
            .iter()
            .enumerate()
            .choose(&mut thread_rng())
        {
            Some((i, &ball)) => {
                self.drawn_balls.push(self.available_balls.remove(i));
                Ok(ball)
            }
            None => Err("No more balls"),
        }
    }
}

fn main() {
    let mut bingo = BingoMachine::new();
    println!("\"d\" to draw, \"s\" to show results, \"a\" to available balls");
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "d" => match bingo.draw_ball() {
                Ok(ball) => println!("{}", ball),
                Err(msg) => {
                    println!("{}", msg);
                    break;
                }
            },
            "s" => println!("{:?}", bingo.drawn_balls()),
            "a" => println!("{:?}", bingo.available_balls()),
            _ => {
                println!("unknown command");
            }
        }
        println!("")
    }
}

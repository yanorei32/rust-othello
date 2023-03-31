#![warn(clippy::pedantic, clippy::nursery)]
#![feature(result_option_inspect)]
#![feature(slice_flatten)]
#![feature(is_some_and)]

use std::io::Write;

mod ai;
mod math;
mod othello;

use math::Coordinate;
use othello::{Board, Player};

fn main() {
    let mut board: Board<8, 8> = Board::new();
    let mut p = Player::First;

    loop {
        println!("{board}");
        println!("Player: {p}");

        if board.is_pass(p) {
            if board.is_pass(!p) {
                // game set!
                break;
            }

            println!("PASS!");
            p = !p;
            continue;
        }

        if p == Player::Second {
            board.put(ai::think(&board, p).unwrap(), p).unwrap();
            p = !p;
            continue;
        }

        let prompt = |name: &str| -> usize {
            let mut s = String::new();
            loop {
                print!("{name}? ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut s).unwrap();
                if let Ok(v) = s.trim().parse::<usize>() {
                    return v;
                }
            }
        };

        let Ok(c) = Coordinate::try_new(prompt("x"), prompt("y")) else {
            println!("Out of range. Please select inside of board.");
            continue;
        };

        if let Err(e) = board.put(c, p) {
            println!("{e}");
            continue;
        }

        p = !p;
    }

    let stat = board.stat();
    print!("{}: {} ", Player::First, stat.first);
    print!("{}: {} ", Player::Second, stat.second);

    match usize::cmp(&stat.first, &stat.second) {
        std::cmp::Ordering::Greater => println!("{} Win!", Player::First),
        std::cmp::Ordering::Equal => println!("DRAW!"),
        std::cmp::Ordering::Less => println!("{} Win!", Player::Second),
    }

    let w = board.record().len().to_string().len();
    for (n, (p, c)) in board.record().iter().enumerate() {
        println!("{n:>w$} {p} {c}");
    }
}

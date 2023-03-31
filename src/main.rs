#![warn(clippy::pedantic, clippy::nursery)]
#![feature(result_option_inspect)]
#![feature(slice_flatten)]
#![feature(is_some_and)]

use std::io::Write;
use std::cmp::Ordering;

mod ai;
mod math;
mod othello;

use math::Coordinate;
use othello::{Board, Player};

fn main() {
    let mut b: Board<8, 8> = Board::new();
    let mut p = Player::First;

    println!("Your Color {p}");

    loop {
        if b.is_pass(p) {
            if b.is_pass(!p) {
                // game set!
                break;
            }

            println!("PASS!");
            p = !p;
            continue;
        }

        if p == Player::Second {
            b.put(ai::think(&b, p).unwrap(), p).unwrap();
            p = !p;
            continue;
        }

        println!("{b}");

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

        if let Err(e) = b.put(c, p) {
            println!("{e}");
            continue;
        }

        p = !p;
    }

    let s = b.stat();
    print!("{}: {} ", Player::First, s.first);
    print!("{}: {} ", Player::Second, s.second);

    match usize::cmp(&s.first, &s.second) {
        Ordering::Greater => println!("{} Win!", Player::First),
        Ordering::Equal => println!("DRAW!"),
        Ordering::Less => println!("{} Win!", Player::Second),
    }

    println!();
    println!("{b}");

    let w = b.record().len().to_string().len();
    for (n, (p, c)) in b.record().iter().enumerate() {
        println!("{n:>w$} {p} {c}");
    }
}

#![feature(result_option_inspect)]
#![feature(slice_flatten)]
#![feature(is_some_and)]

use std::fmt::Display;
use std::io::Write;
use std::ops::Not;

const BOARD_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    #[must_use]
    fn try_new(x: usize, y: usize) -> Result<Self, &'static str> {
        if !(0..BOARD_SIZE).contains(&x) || !(0..BOARD_SIZE).contains(&y) {
            return Err("Invalid Coordinate");
        }

        Ok(Self { x, y })
    }

    #[must_use]
    fn try_add(&self, v: Vector) -> Result<Self, &'static str> {
        let add_isize = |vu: usize, vi: isize| -> Result<usize, &'static str> {
            let vi_abs = usize::try_from(vi.abs()).unwrap();

            if 0 < vi {
                vu.checked_add(vi_abs)
            } else {
                vu.checked_sub(vi_abs)
            }
            .ok_or("Failed to add / sub isize")
        };

        Self::try_new(add_isize(self.x, v.x)?, add_isize(self.y, v.y)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    First,
    Second,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Empty,
    First,
    Second,
}

impl CellState {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

impl From<Player> for CellState {
    fn from(p: Player) -> Self {
        match p {
            Player::First => CellState::First,
            Player::Second => CellState::Second,
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, "-"),
            CellState::First => write!(f, "○"),
            CellState::Second => write!(f, "●"),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    state: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    record: Vec<(Player, Coordinate)>,
}

impl Board {
    fn new() -> Self {
        let mut b = Self {
            state: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            record: Vec::new(),
        };

        let n = BOARD_SIZE / 2;
        b.state[n - 1][n - 1] = Player::First.into();
        b.state[n - 1][n] = Player::Second.into();
        b.state[n][n - 1] = Player::Second.into();
        b.state[n][n] = Player::First.into();

        b
    }

    fn get_cell_mut(&mut self, c: Coordinate) -> &mut CellState {
        &mut self.state[c.y][c.x]
    }

    fn get_cell(&self, c: Coordinate) -> &CellState {
        &self.state[c.y][c.x]
    }

    fn put(&mut self, c: Coordinate, p: Player) -> Result<(), &'static str> {
        if !self.get_cell(c).is_empty() {
            return Err("Cell is not Empty.");
        }

        if !(-1..=1)
            .map(|x| (-1..=1).map(move |y| Vector::new(x, y)))
            .flatten()
            .filter(|v| !v.is_zero())
            .map(|dir| self.flip(c, dir, p))
            .collect::<Vec<Result<usize, ()>>>()
            .iter()
            .any(|&v| v.is_ok_and(|v| 0 < v))
        {
            return Err("Cell is not placable");
        }

        *self.get_cell_mut(c) = p.into();
        self.record.push((p, c));

        Ok(())
    }

    fn flip(&mut self, at: Coordinate, dir: Vector, p: Player) -> Result<usize, ()> {
        // is wall?
        let at = at.try_add(dir).map_err(|_| ())?;
        match *self.get_cell(at) {
            CellState::Empty => Err(()),
            s if s == p.into() => Ok(0),
            s if s == (!p).into() => self
                .flip(at, dir, p)
                .map(|v| v + 1)
                .inspect(|_| *self.get_cell_mut(at) = p.into()),
            _ => unreachable!(),
        }
    }

    fn is_pass(&self, p: Player) -> bool {
        (0..BOARD_SIZE)
            .map(|y| {
                (0..BOARD_SIZE)
                    .map(move |x| unsafe { Coordinate::try_new(x, y).unwrap_unchecked() })
            })
            .flatten()
            .filter(|&c| self.get_cell(c).is_empty())
            .filter(|&c| {
                (-1..=1)
                    .map(|x| (-1..=1).map(move |y| Vector::new(x, y)))
                    .flatten()
                    .filter(|v| !v.is_zero())
                    .map(|dir| self.flipable(c, dir, p))
                    .any(|v| v.is_ok_and(|v| 0 < v))
            })
            .next()
            .is_none()
    }

    fn flipable(&self, at: Coordinate, dir: Vector, p: Player) -> Result<usize, ()> {
        // is wall?
        let at = at.try_add(dir).map_err(|_| ())?;
        match *self.get_cell(at) {
            CellState::Empty => Err(()),
            s if s == p.into() => Ok(0),
            s if s == (!p).into() => self.flipable(at, dir, p).map(|v| v + 1),
            _ => unreachable!(),
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = (BOARD_SIZE - 1).to_string().len();

        write!(f, "{:^width$} ", "", width = width)?;

        for n in 0..BOARD_SIZE {
            write!(f, "{:^width$} ", n, width = width)?;
        }

        writeln!(f)?;

        for y in 0..BOARD_SIZE {
            write!(f, "{y:>width$} ", width = width)?;

            for x in 0..BOARD_SIZE {
                let c = unsafe { Coordinate::try_new(x, y).unwrap_unchecked() };
                write!(
                    f,
                    "{:^width$} ",
                    (*self.get_cell(c)).to_string(),
                    width = width
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    let mut p = Player::Second;

    loop {
        p = !p;
        println!("{board}");
        println!("Player: {}", CellState::from(p));

        if board.is_pass(p) {
            if board.is_pass(!p) {
                println!("GameSet!");
                break;
            }

            println!("PASS!");
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
            println!("Out of range. Please select inside of BOARD_SIZE: {BOARD_SIZE}");
            continue;
        };

        if let Err(e) = board.put(c, p) {
            println!("{e}");
            continue;
        }
    }

    for (p, c) in board.record {
        println!("{}: {:?}", CellState::from(p), c);
    }
}

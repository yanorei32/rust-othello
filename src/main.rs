use std::fmt::Display;
use std::io::Write;
use std::ops::Not;

const BOARD_SIZE: usize = 8;

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
            return Err("Invalid coordinate");
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

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

#[derive(Debug, Clone, Copy)]
struct Board {
    state: [[CellState; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        let mut b = Self {
            state: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        };

        b.state[3][3] = Player::First.into();
        b.state[3][4] = Player::Second.into();
        b.state[4][3] = Player::Second.into();
        b.state[4][4] = Player::First.into();

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
            return Err("Put point is not Empty.");
        }

        *self.get_cell_mut(c) = p.into();

        (-1..=1)
            .map(|x| (-1..=1).map(move |y| (x, y)))
            .flatten()
            .map(|v| Vector::new(v.0, v.1))
            .filter(|v| !v.is_zero())
            .for_each(|dir| {
                self.flip(c, dir, p);
            });

        Ok(())
    }

    fn flip(&mut self, at: Coordinate, dir: Vector, p: Player) -> bool {
        // 盤面の範囲外に出る際に検出してReturn
        let Ok(at) = at.try_add(dir) else {
            return false;
        };

        match *self.get_cell(at) {
            // len > 0 挟まれている色が検索によって存在するかつその先に指定された色があれば挟まれたと判定する
            s if s == p.into() => true,

            // 指定した色とは反対の色を探す.これで挟まれている色を探索する.
            s if s == (!p).into() => {
                let found = self.flip(at, dir, p);

                if found {
                    *self.get_cell_mut(at) = p.into();
                }

                found
            }

            // 何もなかった場合はReturn
            _ => false,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n",
            self.state
                .iter()
                .map(|row| row.map(|v| v.to_string()).join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn main() {
    let mut board = Board::new();
    println!("{board}");

    let mut p = Player::Second;
    loop {
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

        println!("{c}");

        if board.put(c, p).is_err() {
            println!("This is not free space. Please select free space.");
            continue;
        }

        println!("{board}");

        p = !p;
    }
}

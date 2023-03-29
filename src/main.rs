use std::fmt::Display;
use std::ops::Not;

const BOARD_SIZE: usize = 8;

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
        write!(
            f,
            "{}",
            match self {
                CellState::Empty => "-",
                CellState::First => "○",
                CellState::Second => "●",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    state: [[CellState; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            state: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn is_inside_board(x: usize, y: usize) -> bool {
        !(x > BOARD_SIZE - 1 || y > BOARD_SIZE - 1)
    }

    fn is_free_space(&self, x: usize, y: usize) -> Result<bool, &'static str> {
        if !Board::is_inside_board(x, y) {
            return Err("Out of range.");
        }

        Ok(self.state[y][x] == CellState::Empty)
    }

    fn put(&mut self, x: usize, y: usize, p: Player) -> Result<(), &'static str> {
        if !Board::is_inside_board(x, y) {
            return Err("Out of range.");
        }

        if self.is_free_space(x, y)? {
            self.state[y][x] = p.into();
            self.frip(x, y, p);
        } else {
            return Err("Put point is not None.");
        }

        Ok(())
    }

    fn search(
        &self,
        target_x: usize,
        target_y: usize,
        direction_x: isize,
        direction_y: isize,
        p: Player,
        len: usize,
    ) -> usize {
        // 盤面の範囲外に出る際に検出してReturn
        if target_x == BOARD_SIZE - 1
            || target_y == BOARD_SIZE - 1
            || target_x == 0
            || target_y == 0
        {
            return 0;
        }

        if self.state[(target_y as isize + direction_y) as usize]
            [(target_x as isize + direction_x) as usize]
            == (!p).into()
        {
            // 指定した色とは反対の色を探す.これで挟まれている色を探索する.
            self.search(
                (target_x as isize + direction_x) as usize,
                (target_y as isize + direction_y) as usize,
                direction_x,
                direction_y,
                p,
                len + 1,
            )
        } else if len > 0
            && self.state[(target_y as isize + direction_y) as usize]
                [(target_x as isize + direction_x) as usize]
                == p.into()
        {
            // len > 0 挟まれている色が検索によって存在するかつその先に指定された色があれば挟まれたと判定する
            len
        } else {
            // 何もなかった場合はReturn
            0
        }
    }

    fn frip(&mut self, x: usize, y: usize, p: Player) {
        for dir in [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
        ] {
            let len = self.search(x, y, dir[0], dir[1], p, 0);
            let mut pos_x = x as isize;
            let mut pos_y = y as isize;
            for _ in 0..len {
                pos_x += dir[0];
                pos_y += dir[1];
                self.state[pos_y as usize][pos_x as usize] = p.into();
            }
        }
    }

    fn put_first_board(&mut self) {
        self.put(3, 3, Player::First).unwrap();
        self.put(4, 3, Player::Second).unwrap();
        self.put(3, 4, Player::Second).unwrap();
        self.put(4, 4, Player::First).unwrap();
    }

    fn print(&self) {
        self.state.iter().for_each(|row| {
            println!(" {}", row.map(|v| v.to_string()).join(" "));
        });
    }
}

fn main() {
    let mut board = Board::new();
    board.put_first_board();
    board.print();

    let mut p = Player::Second;
    loop {
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).unwrap();
        let mut y = String::new();
        std::io::stdin().read_line(&mut y).unwrap();

        let x = x.trim_end().to_owned().parse().unwrap_or(0);
        let y = y.trim_end().to_owned().parse().unwrap_or(0);

        println!("({x}, {y})");

        if x > BOARD_SIZE - 1 || y > BOARD_SIZE - 1 {
            println!("Out of range. Please select inside of BOARD_SIZE: {BOARD_SIZE}",);
            continue;
        }

        if !board.is_free_space(x, y).unwrap() {
            println!("This is not free space. Please select free space.");
            continue;
        }

        board.put(x, y, p).unwrap();
        board.print();

        p = !p;
    }
}

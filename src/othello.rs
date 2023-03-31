use std::fmt::Display;
use std::ops::Not;

use crate::math::{Coordinate, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::First => write!(f, "○"),
            Player::Second => write!(f, "●"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Stat {
    pub first: usize,
    pub second: usize,
    pub empty: usize,
}

#[derive(Debug, Clone)]
pub struct Board<const SIZE_X: usize, const SIZE_Y: usize> {
    state: [[CellState; SIZE_X]; SIZE_Y],
    record: Vec<(Player, Coordinate<SIZE_X, SIZE_Y>)>,
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Board<SIZE_X, SIZE_Y> {
    pub fn new() -> Self {
        let mut b = Self {
            state: [[CellState::Empty; SIZE_X]; SIZE_Y],
            record: Vec::new(),
        };

        let nx = SIZE_X / 2;
        let ny = SIZE_Y / 2;
        b.state[ny - 1][nx - 1] = Player::First.into();
        b.state[ny - 1][nx] = Player::Second.into();
        b.state[ny][nx - 1] = Player::Second.into();
        b.state[ny][nx] = Player::First.into();

        b
    }

    pub fn stat(&self) -> Stat {
        let mut s = Stat::default();

        self.state.flatten().iter().for_each(|c| match c {
            CellState::Empty => s.empty += 1,
            CellState::First => s.first += 1,
            CellState::Second => s.second += 1,
        });

        s
    }

    #[inline]
    fn get_cell_mut(&mut self, c: Coordinate<SIZE_X, SIZE_Y>) -> &mut CellState {
        &mut self.state[c.y()][c.x()]
    }

    #[inline]
    pub fn get_cell(&self, c: Coordinate<SIZE_X, SIZE_Y>) -> &CellState {
        &self.state[c.y()][c.x()]
    }

    pub fn put(&mut self, c: Coordinate<SIZE_X, SIZE_Y>, p: Player) -> Result<(), &'static str> {
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

    fn flip(
        &mut self,
        at: Coordinate<SIZE_X, SIZE_Y>,
        dir: Vector,
        p: Player,
    ) -> Result<usize, ()> {
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

    pub fn is_pass(&self, p: Player) -> bool {
        (0..SIZE_Y)
            .map(|y| {
                (0..SIZE_X).map(move |x| unsafe { Coordinate::try_new(x, y).unwrap_unchecked() })
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

    pub fn flipable(
        &self,
        at: Coordinate<SIZE_X, SIZE_Y>,
        dir: Vector,
        p: Player,
    ) -> Result<usize, ()> {
        // is wall?
        let at = at.try_add(dir).map_err(|_| ())?;
        match *self.get_cell(at) {
            CellState::Empty => Err(()),
            s if s == p.into() => Ok(0),
            s if s == (!p).into() => self.flipable(at, dir, p).map(|v| v + 1),
            _ => unreachable!(),
        }
    }

    pub fn record(&self) -> &Vec<(Player, Coordinate<SIZE_X, SIZE_Y>)> {
        &self.record
    }
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Display for Board<SIZE_X, SIZE_Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_width = (SIZE_X - 1).to_string().len();
        let y_width = (SIZE_Y - 1).to_string().len();

        write!(f, "{:^w$} ", "", w = y_width)?;

        for n in 0..SIZE_X {
            write!(f, "{:^w$} ", n, w = x_width)?;
        }

        writeln!(f)?;

        for y in 0..SIZE_Y {
            write!(f, "{y:>w$} ", w = y_width)?;

            for x in 0..SIZE_X {
                let c = unsafe { Coordinate::try_new(x, y).unwrap_unchecked() };
                write!(f, "{:^w$} ", (*self.get_cell(c)).to_string(), w = x_width)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

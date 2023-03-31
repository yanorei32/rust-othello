use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate<const SIZE_X: usize, const SIZE_Y: usize> {
    x: usize,
    y: usize,
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Coordinate<SIZE_X, SIZE_Y> {
    #[must_use]
    pub fn try_new(x: usize, y: usize) -> Result<Self, &'static str> {
        if !(0..SIZE_X).contains(&x) || !(0..SIZE_Y).contains(&y) {
            return Err("Invalid Coordinate");
        }

        Ok(Self { x, y })
    }

    #[must_use]
    pub fn try_add(&self, v: Vector) -> Result<Self, &'static str> {
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

    #[inline]
    pub fn x(&self) -> usize {
        self.x
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.y
    }
}

impl<const SIZE_X: usize, const SIZE_Y: usize> Display for Coordinate<SIZE_X, SIZE_Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wx = (SIZE_X - 1).to_string().len();
        let wy = (SIZE_Y - 1).to_string().len();
        write!(f, "[{:>wx$}, {:>wy$}]", self.x, self.y, wx = wx, wy = wy)
    }
}

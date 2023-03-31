use crate::math::{Coordinate, Vector};
use crate::othello::{Board, Player};

pub fn think<const SIZE_X: usize, const SIZE_Y: usize>(
    b: &Board<SIZE_X, SIZE_Y>,
    p: Player,
) -> Option<Coordinate<SIZE_X, SIZE_Y>> {
    (0..SIZE_Y)
        .map(|y| (0..SIZE_X).map(move |x| unsafe { Coordinate::try_new(x, y).unwrap_unchecked() }))
        .flatten()
        .filter(|&c| b.get_cell(c).is_empty())
        .map(|c| {
            (
                (-1..=1)
                    .map(|x| (-1..=1).map(move |y| Vector::new(x, y)))
                    .flatten()
                    .filter(|v| !v.is_zero())
                    .map(|dir| b.flipable(c, dir, p))
                    .filter_map(|b| b.ok())
                    .sum::<usize>(),
                c,
            )
        })
        .max_by_key(|v| v.0)
        .map(|v| v.1)
}

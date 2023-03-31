use crate::math::{Coordinate, Vector};
use crate::othello::{Board, Player};

pub fn think<const SIZE_X: usize, const SIZE_Y: usize>(
    b: &Board<SIZE_X, SIZE_Y>,
    p: Player,
) -> Option<Coordinate<SIZE_X, SIZE_Y>> {
    let candidate: Vec<(usize, Coordinate<SIZE_X, SIZE_Y>)> = (0..SIZE_Y)
        .flat_map(|y| {
            (0..SIZE_X).map(move |x| unsafe { Coordinate::try_new(x, y).unwrap_unchecked() })
        })
        .filter(|&c| b.get_cell(c).is_empty())
        .map(|c| {
            (
                (-1..=1)
                    .flat_map(|x| (-1..=1).map(move |y| Vector::new(x, y)))
                    .filter(|v| !v.is_zero())
                    .map(|dir| b.flipable(c, dir, p))
                    .filter_map(std::result::Result::ok)
                    .sum::<usize>(),
                c,
            )
        })
        .filter(|&(count, _)| count != 0)
        .collect();

    candidate
        .iter()
        .find(|v| v.1.is_corner())
        .or_else(|| candidate.iter().max_by_key(|v| v.0))
        .map(|v| v.1)
}

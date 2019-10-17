//! Implementation of a special Breadth First Search algorithm specialised on
//! the applications in the forbidden island.

use crate::math::{Rect, Vec2};
use std::fmt;

pub enum Error {
    UnavailableStartingPosition
}

/// # Arguments
///
/// * `data` - The data to analyse
/// * `limits` - The Rectangle that must not be left while performing the
///   breadth first search
/// * `start_pos` - The position from which to start bfs, or `None` for any
///   non-`None` start point
/// * `start_marker` - The marker that should be placed at the starting pos of
///   bfs
/// * `marker` - Function that marks all positions which have been reached based
///   on the tile it has been reached from
/// * `output` - Function to convert the markers to the expected output
pub fn bfs<D, I, M, O, F, G>(
    data: &Vec<Vec<Option<D>>>,
    start_pos: Option<Vec<I>>,
    start_marker: Option<M>,
    limits: Option<Rect<I>>,
    marker: F,
    output: G
) -> Result<Vec<Vec<O>>, Error>
where
    I: Copy + fmt::Debug + PartialEq + 'static,
    O: Clone,
    M: Clone,
    F: Fn((Vec2<I>, &Option<M>), (Vec2<I>, &Option<D>)) -> Option<M>,
    G: Fn(Vec2<I>, &Option<D>) -> Option<O>
{
    // Set the starting position or find a possible starting position
    let start_pos = match start_pos {
        Some(start_pos) => start_pos,
        None => unimplemented!()
    };

    // Create the marking working Vector
    let mut marked: Vec<Vec<Option<M>>> = create_none_vec_with_sizes(&data);

    unimplemented!()
}

// Helper function to create a two-dimensional array, with the same amount of
// rows of the provided data vector and the same amount of elements in those
// rows, which will all be set to `None`.
fn create_none_vec_with_sizes<I, O: Clone>(data: &Vec<Vec<Option<I>>>) -> Vec<Vec<Option<O>>> {
    let mut out = Vec::with_capacity(data.len());
    for (i, v) in data.iter().enumerate() {
        out[i] = vec![None; v.len()];
    }

    out
}

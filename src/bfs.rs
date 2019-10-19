//! Implementation of a special Breadth First Search algorithm specialised on
//! the applications in the forbidden island.

use crate::flat_2dvec;
use crate::map::{BlackWhite as BlackWhiteMap, FieldPos};
use crate::math::{Rect, Vec2};

pub enum Error {
    UnavailableStartingPosition
}

/// Find the positions that can be reached with a normal move set from the
/// provided starting position.
///
/// # Arguments
/// `data` - Black and white map of the island that should be tested
/// `start_pos` - The position from which to start, or `None` if any position
/// will suffice
///
/// # Returns
/// Map where all tiles which are island tiles and can be reached are `true` and
/// the others `false`
pub fn reachable_positions(
    data: &BlackWhiteMap,
    start_pos: Option<FieldPos>
) -> Result<Vec<Vec<bool>>, Error> {
    let start_pos = match start_pos {
        Some(start_pos) => Some(Vec2::from_values(
            start_pos.x as usize,
            start_pos.y as usize
        )),
        None => None
    };
    let limits = Some(Rect::from_slice([
        data.limit_rect().x as usize,
        data.limit_rect().y as usize,
        data.limit_rect().w as usize,
        data.limit_rect().h as usize
    ]));

    // Mark the island tiles (`true`) which have been reached with a Some. Every
    // reached tile marked in such a way will then produce a `true` in the
    // output, all others `false`.
    bfs(
        data,
        start_pos,
        (),
        limits,
        |_, (_, tile)| {
            if let Some(true) = tile {
                Some(())
            }
            else {
                None
            }
        },
        |_, marker| if marker.is_some() { true } else { false }
    )
}

/// # Breadth first search
/// Generic general purpose implementation of the breadth first search
/// algorithm. Can be used as a basis for other algorithms that use breadth
/// first search.
///
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
pub fn bfs<D, O, M, F, G>(
    data: &Vec<Vec<Option<D>>>,
    start_pos: Option<Vec2<usize>>,
    start_marker: M,
    limits: Option<Rect<usize>>,
    marker: F,
    output: G
) -> Result<Vec<Vec<O>>, Error>
where
    O: Clone,
    M: Clone + PartialEq,
    F: Fn((Vec2<usize>, &Option<M>), (Vec2<usize>, &Option<D>)) -> Option<M>,
    G: Fn(Vec2<usize>, &Option<M>) -> O
{
    // Set the starting position or find a possible starting position
    let start_pos = match start_pos {
        Some(start_pos) => start_pos,
        None => {
            // Find a non-None item in the 2d-vector. This must exist, otherwise it is an
            // error.
            match flat_2dvec::flatten(data)
                .into_iter()
                .find(|(_, e)| e.is_some())
            {
                Some((pos, _)) => pos,
                None => return Err(Error::UnavailableStartingPosition)
            }
        }
    };

    // Create the marking working Vector and mark the starting position
    let mut marked: Vec<Vec<Option<M>>> = Vec::with_capacity(data.len());
    for line in data {
        marked.push(vec![None; line.len()]);
    }
    marked[start_pos.y][start_pos.x] = Some(start_marker);

    let mut marked_flat = flat_2dvec::flatten_mut(&mut marked);

    // Mark all places reachable from the starting position with the marking
    // function.
    loop {
        let mut something_changed = false;

        // Go through the map and mark all positions that result from the marking of the
        // last iteration.
        for (pos, e) in &mut marked_flat {
            // Look in the four primary directions
            for nb in pos.neighbours(limits) {
                let to_assign = marker((*pos, e), (nb, &data[nb.y][nb.x]));
                if to_assign != **e {
                    **e = to_assign;
                    something_changed = true;
                }
            }
        }

        // If nothing changed in an iteration, it will in no further iteration, so the
        // bfs is done.
        if !something_changed {
            break;
        }
    }

    // Convert the markers into the requested output format.
    let mut out_map = Vec::with_capacity(marked.len());
    for (y, marked_line) in marked.iter().enumerate() {
        let mut out_line = Vec::with_capacity(marked_line.len());
        for (x, e) in marked_line.iter().enumerate() {
            out_line.push(output(Vec2::from_values(x, y), e));
        }
        out_map.push(out_line);
    }

    Ok(out_map)
}

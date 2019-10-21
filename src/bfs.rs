//! Implementation of a special Breadth First Search algorithm specialised on
//! the applications in the forbidden island.

use crate::map::{BlackWhite as BlackWhiteMap, FieldPos, Map, MapExt};
use crate::math::Vec2;

#[derive(Debug)]
pub enum Error {
    UnavailableStartingPosition,
    StartingPosOutOfBounds
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
) -> Result<Map<bool>, Error> {
    // Mark the island tiles (`true`) which have been reached with a Some. Every
    // reached tile marked in such a way will then produce a `true` in the
    // output, all others `false`.
    bfs(
        data,
        start_pos,
        (),
        |_, (_, &tile)| {
            if tile {
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
/// * `start_pos` - The position from which to start bfs, or `None` for any
///   starting position that is standable
/// * `start_marker` - The marker that should be placed at the starting pos of
///   bfs
/// * `marker` - Function that marks all positions which have been reached based
///   on the tile it has been reached from
/// * `output` - Function to convert the markers to the expected output
pub fn bfs<T, O, M, F, G>(
    data: &Map<T>,
    start_pos: Option<FieldPos>,
    start_marker: M,
    marker: F,
    output: G
) -> Result<Map<O>, Error>
where
    Map<T>: MapExt,
    O: Clone,
    M: Clone + PartialEq,
    F: Fn((FieldPos, &Option<M>), (FieldPos, &T)) -> Option<M>,
    G: Fn(FieldPos, &Option<M>) -> O
{
    // Set the starting position or find a possible starting position
    let start_pos = match start_pos {
        Some(start_pos) => start_pos,
        None => {
            // Find a non-None item in the 2d-vector. This must exist, otherwise it is an
            // error.
            match data.iter().find(|(pos, _)| data.is_standable(*pos)) {
                Some((pos, _)) => Vec2::from_values(pos.x as u8, pos.y as u8),
                None => return Err(Error::UnavailableStartingPosition)
            }
        }
    };

    // Create the marking working Vector and mark the starting position
    let mut marked: Map<Option<M>> = Map::new(data.size(), None);

    if !data.limit_rect().contains(start_pos) {
        return Err(Error::StartingPosOutOfBounds);
    }
    marked.set(start_pos, Some(start_marker));

    // Mark all places reachable from the starting position with the marking
    // function.
    loop {
        let mut something_changed = false;

        // Go through the map and mark all positions that result from the marking of the
        // last iteration.
        for y in 0..data.height() {
            for x in 0..data.width() {
                let pos = Vec2::from_values(x, y);
                // Look in the four primary directions
                for nb in pos.neighbours(None) {
                    let target_point_data = match data.get(nb) {
                        Some(e) => e,
                        None => break
                    };
                    let to_assign =
                        marker((pos, &marked.get(pos).unwrap()), (nb, target_point_data));
                    if &to_assign != marked.get(nb).unwrap() {
                        marked.set(nb, to_assign);
                        something_changed = true;
                    }
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
    let mut out_map = Vec::with_capacity(data.height() as usize);
    for y in 0..data.height() {
        let mut out_line = Vec::with_capacity(data.width() as usize);
        for x in 0..data.width() {
            let pos = Vec2::from_values(x, y);
            out_line.push(output(pos, marked.get(pos).unwrap()));
        }
        out_map.push(out_line);
    }

    Ok(out_map.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::Rect;
    use std::ops::Deref;

    impl MapExt for Map<Option<u32>> {
        fn is_standable(&self, pos: FieldPos) -> bool { self.get(pos).unwrap().is_some() }

        fn limit_rect(&self) -> Rect<u8> { Rect::from_slice([0, 0, self.width(), self.height()]) }
    }

    #[test]
    fn bfs_cross_no_start_point() {
        let data: Map<Option<u32>> = vec![
            vec![None, None, Some(1), None, None, None],
            vec![None, Some(2), Some(3), Some(4), None, None],
            vec![None, None, Some(5), None, None, None],
        ]
        .into();

        let bfs = bfs(
            &data,
            None,
            true,
            |_, (_, tile)| {
                if let Some(_) = tile {
                    Some(true)
                }
                else {
                    None
                }
            },
            |_, marker| if let Some(true) = marker { true } else { false }
        )
        .unwrap();

        let expected = vec![
            vec![false, false, true, false, false, false],
            vec![false, true, true, true, false, false],
            vec![false, false, true, false, false, false],
        ];

        assert_eq!(&expected, bfs.deref());
    }
}

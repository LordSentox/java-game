//! Conversion of two-dimensional Vectors from and to one-dimensional vectors
//! holding the positional info of it.

use crate::math::Vec2;

pub fn flatten<'a, T>(data: &'a Vec<Vec<T>>) -> Vec<(Vec2<usize>, &'a T)> {
    // Find out the length of the resulting vector, so the allocation goes faster.
    let len = flat_len(data);

    let mut flattened = Vec::with_capacity(len);
    for (y, line) in data.iter().enumerate() {
        for (x, e) in line.iter().enumerate() {
            flattened.push((Vec2::from_values(x, y), e));
        }
    }

    flattened
}

pub fn flatten_mut<'a, T>(data: &'a mut Vec<Vec<T>>) -> Vec<(Vec2<usize>, &'a mut T)> {
    // Find out the length of the resulting vector, so the allocation goes faster.
    let len = flat_len(data);

    let mut flattened = Vec::with_capacity(len);
    for (y, line) in data.iter_mut().enumerate() {
        for (x, e) in line.iter_mut().enumerate() {
            flattened.push((Vec2::from_values(x, y), e));
        }
    }

    flattened
}

/// Find out the number of elements in all inner vecs combined.
///
/// # Example
///
/// ```
/// let vec = vec![
///     vec![1., 0., 0.]
///     vec![2., 3., 4.]
/// ];
///
/// assert_eq!(6, flat_len(&vec));
/// ```
pub fn flat_len<T>(vec: &Vec<Vec<T>>) -> usize {
    let mut len = 0;
    for line in vec {
        for _ in line {
            len += 1;
        }
    }
    len
}

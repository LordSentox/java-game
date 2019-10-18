//! Traits for iterating through a 2D-Vector.

use crate::math::Vec2;

pub struct Iter2d<'a, T> {
    internal: &'a Vec<Vec<T>>,
    pos:      Vec2<usize>
}

impl<'a, T> Iter2d<'a, T> {
    pub fn new(source: &'a Vec<Vec<T>>) -> Self {
        Self {
            internal: source,
            pos:      Vec2::new()
        }
    }
}

impl<'a, T> Iterator for Iter2d<'a, T> {
    type Item = (Vec2<usize>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the current line the Iterator is on.
        if let Some(line) = self.internal.get(self.pos.y) {
            // Get the current element
            if let Some(e) = line.get(self.pos.x) {
                // Try to shift the pointer one to the right and return the current element.
                let old_pos = self.pos;
                self.pos.x += 1;

                Some((old_pos, &e))
            }
            else {
                // There are no items left on the current line. Select the next line and return
                // the next item recursively.
                self.pos.y += 1;
                self.pos.x = 0;
                self.next()
            }
        }
        else {
            // There are no lines left.
            None
        }
    }
}

//! Data type used to store the treasure cards and the flood cards respectively.

use rand::{seq::SliceRandom, thread_rng};

/// Card Stack to store the draw stack and the discard stack of a card type.
pub struct CardStack<T> {
    draw_stack:    Vec<T>,
    discard_stack: Vec<T>
}

impl<T> CardStack<T> {
    /// Create a new card stack with the given draw stack. Does not shuffle the
    /// cards. Must be done manually, so the card stack can be loaded from a
    /// file if needed.
    ///
    /// # Param
    /// `draw_stack` - The initial draw stack. The last card in the vector will
    /// be the top card
    pub fn new(draw_stack: Vec<T>) -> Self {
        let stack_size = draw_stack.len();
        Self {
            draw_stack,
            discard_stack: Vec::with_capacity(stack_size)
        }
    }

    /// Get the discard pile, which may be looked into for both flood cards and
    /// treasure cards.
    pub fn discard_stack(&self) -> &Vec<T> { &self.discard_stack }

    /// Draw a card from the draw stack and return it. Returns `None`, if there
    /// are no cards on the draw stack left.
    pub fn draw_card(&mut self) -> Option<T> { self.draw_stack.pop() }

    /// Discarding a card means to put it on the discard pile. If you want to
    /// throw it away, just drop it or let it go out of scope.
    pub fn discard_card(&mut self, card: T) { self.discard_stack.push(card) }

    /// Shuffle the draw stack. The discard pile is not touched.
    pub fn shuffle(&mut self) { self.draw_stack.shuffle(&mut thread_rng()); }

    /// Shuffle the discard pile and put it on top of the draw stack. The draw
    /// stack does not get shuffled.
    pub fn shuffle_back(&mut self) {
        self.discard_stack.shuffle(&mut thread_rng());
        self.draw_stack.append(&mut self.discard_stack);
    }

    /// Get the amount of cards that are left on the draw stack.
    pub fn draw_stack_size(&self) -> usize { self.draw_stack.len() }

    /// Get the amount of cards that are in the discard pile.
    pub fn discard_stack_size(&self) -> usize { self.discard_stack.len() }

    /// Get the amount of cards left in the draw pile and the discard pile
    /// combined.
    pub fn size(&self) -> usize { self.draw_stack_size() + self.discard_stack_size() }
}

#[cfg(test)]
mod test {
    use super::CardStack;

    #[test]
    fn new() {
        // Create a card stack and check, that the draw stack is not shuffled and the
        // discard-stack is empty
        let mut stack = CardStack::new((0..20_u8).collect());

        for i in 20..0 {
            assert_eq!(i, stack.draw_card().unwrap());
        }
        assert_eq!(0, stack.discard_stack_size());
    }

    #[test]
    fn draw_card() {
        let mut stack = CardStack::new(vec![1]);
        assert_eq!(Some(1), stack.draw_card());
        assert_eq!(None, stack.draw_card());
    }

    #[test]
    fn discard_card() {
        let mut stack = CardStack::new(vec![1, 2]);
        let card = stack.draw_card().unwrap();
        stack.discard_card(card);
        let card = stack.draw_card().unwrap();
        stack.discard_card(card);

        assert_eq!(&vec![2, 1], stack.discard_stack());
    }
}

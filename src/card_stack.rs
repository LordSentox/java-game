//! Data type used to store the treasure cards and the flood cards respectively.

/// Card Stack to store the draw stack and the discard stack of a card type.
pub struct CardStack<T> {
    draw_stack:    Vec<T>,
    discard_stack: Vec<T>
}

impl<T> CardStack<T> {
    /// Create a new card stack with the given draw stack. Does not shuffle the
    /// cards. Must be done manually, so the card stack can be loaded from a
    /// file if needed.
    pub fn new(_draw_stack: Vec<T>) -> Self { unimplemented!() }

    /// Get the discard pile, which may be looked into for both flood cards and
    /// treasure cards.
    pub fn discard_stack(&self) -> &Vec<T> { &self.discard_stack }

    /// Draw a card from the draw stack.
    pub fn draw_card(&mut self) -> T { unimplemented!() }

    /// Discarding a card means to put it on the discard pile. If you want to
    /// throw it away, just drop it or let it go out of scope.
    pub fn discard_card(&mut self, _card: T) { unimplemented!() }

    /// Shuffle the draw stack. The discard pile is not touched.
    pub fn shuffle(&mut self) { unimplemented!() }

    /// Shuffle the discard pile and put it on top of the draw stack. The draw
    /// stack does not get shuffled.
    pub fn shuffle_back(&mut self) { unimplemented!() }

    /// Get the amount of cards that are left on the draw stack.
    pub fn draw_stack_size(&self) -> usize { self.draw_stack.len() }

    /// Get the amount of cards that are in the discard pile.
    pub fn discard_stack_size(&self) -> usize { self.discard_stack.len() }

    /// Get the amount of cards left in the draw pile and the discard pile
    /// combined.
    pub fn size(&self) -> usize { self.draw_stack_size() + self.discard_stack_size() }
}

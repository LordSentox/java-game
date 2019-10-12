pub struct CardStack<T: !Copy> {
    draw_stack: Vec<T>,
    discard_stack: Vec<T>
}

impl<T> CardStack<T> {
    pub fn new() -> Self<T> {
        unimplemented!()
    }

    pub fn discard_stack(&self) -> &Vec<T> {
        &self.discard_stack
    }

    pub fn draw_card(&mut self) -> T {
        unimplemented!()
    }

    pub fn discard_card(&mut self, card: T) {
        unimplemented!()
    }

    pub fn shuffle_back(&mut self) {
        unimplemented!()
    }
}

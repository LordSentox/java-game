pub struct CardStack<T> {
    draw_stack:    Vec<T>,
    discard_stack: Vec<T>
}

impl<T> CardStack<T> {
    pub fn new() -> Self { unimplemented!() }

    pub fn discard_stack(&self) -> &Vec<T> { &self.discard_stack }

    pub fn draw_card(&mut self) -> T { unimplemented!() }

    pub fn discard_card(&mut self, _card: T) { unimplemented!() }

    pub fn shuffle_back(&mut self) { unimplemented!() }
}

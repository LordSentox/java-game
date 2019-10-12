pub struct Courier;

impl CardTransferer for Courier {
    fn can_transfer(_: (u8, u8), _: (u8, u8)) -> bool {
        true
    }
}
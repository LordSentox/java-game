pub trait CardTransferer {
    fn can_transfer(pos: (u8, u8), other_pos: (u8, u8)) -> bool { pos == other_pos }
}

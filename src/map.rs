pub type Full = Map<Tile>;
pub type BlackWhite = Map<bool>;

pub struct Map<T> {
    tiles: Vec<Vec<Option<T>>>
}

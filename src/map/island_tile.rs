/// The different states an island can be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IslandTileState {
    /// The island is dry. Players can freely do anything on these tiles.
    Dry,
    /// Flooded tiles can be drained. Players can still freely do anything on
    /// these tiles, however they should be careful since they will be
    /// [Gone](IslandTileState::Gone) the next time their flood card is
    /// drawn.
    Flooded,
    /// The tile is gone. The diver can still swim through it, but other than
    /// that it's unusable.
    Gone
}

/// Represents one of 24 island map tiles.
#[derive(Clone, Debug, PartialEq)]
pub struct IslandTile {
    state: IslandTileState
}

impl IslandTile {
    pub fn new() -> Self {
        Self {
            state: IslandTileState::Dry
        }
    }

    pub fn state(&self) -> IslandTileState { self.state }

    pub fn set_state(&mut self, state: IslandTileState) { self.state = state; }
}

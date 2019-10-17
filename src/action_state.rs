//! Action State of the currently active turn. A turn starts with the player's
//! adventurer taking up to three actions. Afterwards, they may draw two cards
//! from the treasure deck cards and finally the appropriate amount of flood
//! cards depending on the current [water level](water_level::WaterLevel).

/// The current action state. Contains the three phases and holds an [u8](u8)
/// for the number of actions the player can still take in case of
/// [PlayerAction](ActionState::PlayerAction) or the amount of cards that have
/// to be drawn in case of the other options.
pub enum ActionState {
    PlayerAction(u8),
    DrawArtefactCards(u8),
    DrawFloodCards(u8)
}

impl ActionState {}

use super::*;

macro_rules! call_func {
    ($a:expr, $f:ident) => {
        match $a {
            AdventurerType::Courier => Courier::$f(),
            AdventurerType::Diver => Diver::$f(),
            AdventurerType::Engineer => Engineer::$f(),
            AdventurerType::Explorer => Explorer::$f(),
            AdventurerType::Navigator => Navigator::$f(),
            AdventurerType::Pilot => Pilot::$f()
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdventurerType {
    Courier = 0,
    Diver,
    Engineer,
    Explorer,
    Navigator,
    Pilot
}

impl AdventurerType {
    /// Create a new adventurer corresponding to this adventurer type
    pub fn create(&self) -> Box<dyn Adventurer> {
        match self {
            Self::Courier => Box::new(Courier::new()),
            Self::Diver => Box::new(Diver::new()),
            Self::Engineer => Box::new(Engineer::new()),
            Self::Explorer => Box::new(Explorer::new()),
            Self::Navigator => Box::new(Navigator::new()),
            Self::Pilot => Box::new(Pilot::new())
        }
    }

    /// Returns true, if the player does not have to do anything to activate the
    /// special ability of the adventurer. This does not mean the
    /// special_moves function should be omitted when querying for all
    /// movement options.
    fn implicit_special(&self) -> bool { call_func!(&self, implicit_special) }

    /// Returns if the player is in principle capable of moving others.
    fn can_move_others(&self) -> bool { call_func!(&self, can_move_others) }
}

use amethyst::ecs::{Component, DenseVecStorage};

use crate::adventurer::AdventurerType;
use crate::artefact_type::ArtefactType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IslandTileInfo {
    CaveOfShadows = 0,
    FoolsLanding,
    CliffsOfAbandon,
    BreakersBridge,
    CrimsonForest,
    TwighlightHorrow,
    Watchtower,
    MistyMarsh,
    Observatory,
    DunesOfDeception,
    LostLagoon,
    PhantomRock,
    GoldGate,
    IronGate,
    BronzeGate,
    CopperGate,
    SilverGate,
    TempleOfTheMoon,
    TempleOfTheSun,
    WhisperingGarden,
    HowlingGarden,
    CoralPalace,
    TidalPalace,
    CaveOfAmbers
}

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
#[derive(Copy, Clone, Component, Debug, PartialEq)]
pub struct IslandTile {
    state: IslandTileState,
    info:  IslandTileInfo
}

impl IslandTileInfo {
    pub fn hidden_artefact(self) -> Option<ArtefactType> {
        match self {
            Self::CaveOfShadows => Some(ArtefactType::Fire),
            Self::CaveOfAmbers => Some(ArtefactType::Fire),
            Self::TempleOfTheMoon => Some(ArtefactType::Earth),
            Self::TempleOfTheSun => Some(ArtefactType::Earth),
            Self::WhisperingGarden => Some(ArtefactType::Air),
            Self::HowlingGarden => Some(ArtefactType::Air),
            Self::CoralPalace => Some(ArtefactType::Water),
            Self::TidalPalace => Some(ArtefactType::Water),
            _ => None
        }
    }

    pub fn player_spawn(self) -> Option<AdventurerType> {
        match self {
            Self::FoolsLanding => Some(AdventurerType::Pilot),
            Self::GoldGate => Some(AdventurerType::Navigator),
            Self::IronGate => Some(AdventurerType::Diver),
            Self::BronzeGate => Some(AdventurerType::Engineer),
            Self::CopperGate => Some(AdventurerType::Explorer),
            Self::SilverGate => Some(AdventurerType::Courier),
            _ => None
        }
    }
}

impl IslandTile {
    pub fn new(info: IslandTileInfo) -> Self {
        Self {
            state: IslandTileState::Dry,
            info
        }
    }

    pub fn state(self) -> IslandTileState { self.state }

    pub fn set_state(&mut self, state: IslandTileState) { self.state = state; }

    pub fn info(&self) -> &IslandTileInfo { &self.info }
}

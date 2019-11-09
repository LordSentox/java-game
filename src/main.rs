#![allow(dead_code)]

//! # \[J\]ust \[a\]nother \[v\]irtual \[a\]dventure game
//!
//! Originating from the Software Praktikum at TU Dortmund and written in Java,
//! this is a Rust rewrite of the computer game based on the board game "The
//! Forbidden Island". It is purely for educational purposes and fun.

#[macro_use]
extern crate java_game_derive;

pub mod action_state;
pub mod adventurer;
pub mod artefact_type;
pub mod asset;
pub mod bfs;
pub mod card_stack;
pub mod character;
pub mod difficulty;
pub mod direction;
pub mod iter_2d;
pub mod map;
pub mod math;
pub mod positionable;
pub mod state;
pub mod system;
pub mod util;
pub mod water_level;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir
};
use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with(
            system::CharacterTransformUpdate,
            "character_transform_update_system",
            &[]
        );

    let mut game = Application::build(assets_dir, state::Game)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144
        )
        .build(game_data)?;
    game.run();

    Ok(())
}

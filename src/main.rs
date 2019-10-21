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
pub mod bfs;
pub mod card_stack;
pub mod character;
pub mod difficulty;
pub mod direction;
pub mod flat_2dvec;
pub mod iter_2d;
pub mod map;
pub mod math;
pub mod positionable;
pub mod state;
pub mod system;
pub mod water_level;

fn main() {}

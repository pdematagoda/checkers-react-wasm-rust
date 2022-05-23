use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub const BOARD_LENGTH: i8 = 8;
pub const BOARD_WIDTH: i8 = 8;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Colour {
    Black,
    White,
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnitType {
    Pawn,
    King,
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unit {
    pub colour: Colour,
    pub unit_type: UnitType,
    pub coordinate: Coordinate,
    pub active: bool,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Side {
    pub one: Unit,
    pub two: Unit,
    pub three: Unit,
    pub four: Unit,
    pub five: Unit,
    pub six: Unit,
    pub seven: Unit,
    pub eight: Unit,
    pub nine: Unit,
    pub ten: Unit,
    pub eleven: Unit,
    pub twelve: Unit,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Board {
    pub white_pieces: Side,
    pub black_pieces: Side,
    pub human_player: Colour,
}

#[derive(Clone)]
pub struct InternalBoard {
    pub active_pieces: HashMap<String, Unit>,
    pub human_player: Colour,
}

#[wasm_bindgen]
pub struct PossibleMoves {
    pub first: Option<Coordinate>,
    pub second: Option<Coordinate>,
}

pub fn get_key_for_x_and_y(x: i8, y: i8) -> String {
    let key = format!("{}{}", x, y);

    key.to_string()
}

pub fn get_key_for_unit(unit: &Unit) -> String {
    get_key_for_x_and_y(unit.coordinate.x, unit.coordinate.y)
}

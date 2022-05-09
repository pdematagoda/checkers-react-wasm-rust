use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub const BoardLength: i8 = 8;
pub const BoardWidth: i8 = 8;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum UnitType {
    Pawn,
    King,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
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

pub fn get_key_for_x_and_y(x: i8, y: i8) -> String {
    let key = format!("{}{}", x, y);

    key.to_string()
}

pub fn get_key_for_unit(unit: &Unit) -> String {
    get_key_for_x_and_y(unit.coordinate.x, unit.coordinate.y)
}

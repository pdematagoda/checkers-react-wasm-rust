use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Colour {
    Black,
    White
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum UnitType {
    Pawn,
    King
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Unit {
    pub colour: Colour,
    pub unit_type: UnitType,
    pub coordinate: Coordinate,
    pub active: bool
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
    pub twelve: Unit
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Board {
    pub white_pieces: Side,
    pub black_pieces: Side,
}
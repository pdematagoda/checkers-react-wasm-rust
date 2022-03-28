use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

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

fn generate_pawn(colour: Colour, x: u8, y: u8) -> Unit {
    Unit {
        colour: colour,
        unit_type: UnitType::Pawn,
        active: true,
        coordinate: Coordinate {
            x: x,
            y: y
        }
    }
}

fn generate_pieces(colour: Colour) -> Side {
    match colour {
        Colour::White => Side {
            one: generate_pawn(colour, 1, 1),
            two: generate_pawn(colour, 1, 3),
            three: generate_pawn(colour, 2, 2),
            four: generate_pawn(colour, 3, 1),
            five: generate_pawn(colour, 3, 3),
            six: generate_pawn(colour, 4, 2),
            seven: generate_pawn(colour, 5, 1),
            eight: generate_pawn(colour, 5, 3),
            nine: generate_pawn(colour, 6, 2),
            ten: generate_pawn(colour, 7, 1),
            eleven: generate_pawn(colour, 7, 3),
            twelve: generate_pawn(colour, 8, 1)
        },
        Colour::Black => Side {
            one: generate_pawn(colour, 1, 7),
            two: generate_pawn(colour, 2, 8),
            three: generate_pawn(colour, 2, 6),
            four: generate_pawn(colour, 3, 7),
            five: generate_pawn(colour, 4, 8),
            six: generate_pawn(colour, 4, 6),
            seven: generate_pawn(colour, 5, 7),
            eight: generate_pawn(colour, 6, 8),
            nine: generate_pawn(colour, 6, 6),
            ten: generate_pawn(colour, 7, 7),
            eleven: generate_pawn(colour, 8, 8),
            twelve: generate_pawn(colour, 8, 6)
        }
    }
}

#[wasm_bindgen]
pub fn generateBoard() -> Board {
    Board {
        white_pieces: generate_pieces(Colour::White),
        black_pieces: generate_pieces(Colour::Black)
    }
}

#[wasm_bindgen]
pub fn doMove(board: Board, unit: Unit, to_x: u8, to_y: u8) -> Board {
    let mut resulting_board = board.clone();

    resulting_board
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

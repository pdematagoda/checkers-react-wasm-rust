use crate::logic::models::{Board, Colour, Coordinate, Side, Unit, UnitType, WinningSide};

fn generate_pawn(colour: Colour, x: i8, y: i8) -> Unit {
    Unit {
        colour: colour,
        unit_type: UnitType::Pawn,
        active: true,
        coordinate: Coordinate { x: x, y: y },
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
            twelve: generate_pawn(colour, 8, 2),
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
            twelve: generate_pawn(colour, 8, 6),
        },
    }
}

pub fn generate_board() -> Board {
    Board {
        white_pieces: generate_pieces(Colour::White),
        black_pieces: generate_pieces(Colour::Black),
        human_player: Colour::Black,
        winning_side: WinningSide::None,
    }
}

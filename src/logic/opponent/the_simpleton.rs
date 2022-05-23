use crate::logic::opponent::shared::{get_active_pieces_for_side, get_pieces_with_possible_moves};
use crate::logic::movement::move_generation::get_valid_moves;
use crate::logic::opponent::types::{Opponent, OpponentMove};
use js_sys::Math::random;
use web_sys::console;

use crate::logic::models::{Colour, Coordinate};
use crate::Unit;
use std::collections::HashMap;

fn find_next_move(
    active_pieces: &HashMap<String, Unit>,
    pieces_to_move: Vec<Unit>,
) -> (Unit, Coordinate) {
    let piece_index = (random() * 100.0) as usize % pieces_to_move.len();
    let piece_to_move = pieces_to_move[piece_index];

    let possible_moves = get_valid_moves(&active_pieces, &piece_to_move);

    if possible_moves.len() == 0 {
        console::log_1(&"Need to try again.....".into());

        let mut new_pieces_to_move = pieces_to_move.clone();

        new_pieces_to_move.remove(piece_index);

        return find_next_move(active_pieces, new_pieces_to_move);
    }

    let selected_move = possible_moves[(random() * 100.0) as usize % possible_moves.len()];

    (piece_to_move, selected_move)
}

fn get_opposing_move(
    active_pieces: &HashMap<String, Unit>,
    moved_piece: &Unit,
) -> (Unit, Coordinate) {
    let opposing_colour = match moved_piece.colour {
        Colour::Black => Colour::White,
        Colour::White => Colour::Black,
    };

    console::log_1(&"Moving opposing unit".into());

    let opposing_pieces = get_active_pieces_for_side(&active_pieces, opposing_colour);

    find_next_move(
        &active_pieces,
        get_pieces_with_possible_moves(&active_pieces, opposing_pieces),
    )
}

pub struct TheSimpleton {}

impl Opponent for TheSimpleton {
    fn get_opposing_move(
        &self,
        active_pieces: &HashMap<String, Unit>,
        moved_piece: &Unit,
    ) -> OpponentMove {
        let (piece_to_move, selected_move) = get_opposing_move(active_pieces, moved_piece);

        OpponentMove {
            move_to: selected_move,
            piece: piece_to_move,
        }
    }
}

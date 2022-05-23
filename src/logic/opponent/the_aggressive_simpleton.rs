use crate::logic::opponent::shared::{
    get_active_pieces_for_side, get_map_of_pieces_to_valid_moves, get_pieces_with_possible_moves,
    is_jump_move,
};
use crate::logic::opponent::types::{Opponent, OpponentMove};
use js_sys::Math::random;
use web_sys::console;

use crate::logic::models::{Colour, Coordinate};
use crate::Unit;
use std::collections::HashMap;

struct MoveForPiece {
    pub piece: Unit,
    pub move_to: Coordinate,
}

fn find_next_move(
    active_pieces: &HashMap<String, Unit>,
    pieces_to_move: Vec<Unit>,
) -> (Unit, Coordinate) {
    let moves_to_pieces = get_map_of_pieces_to_valid_moves(active_pieces, pieces_to_move);
    let mut single_move_list: Vec<MoveForPiece> = Vec::new();
    let mut jump_move_list: Vec<MoveForPiece> = Vec::new();

    for (piece, moves) in moves_to_pieces {
        for piece_move in moves.iter() {
            if is_jump_move(&piece, piece_move) {
                jump_move_list.push(MoveForPiece {
                    piece,
                    move_to: *piece_move,
                })
            } else {
                single_move_list.push(MoveForPiece {
                    piece,
                    move_to: *piece_move,
                })
            }
        }
    }

    let moves_to_try = match jump_move_list.len() > 0 {
        true => jump_move_list,
        false => single_move_list,
    };

    let move_index = (random() * 100.0) as usize % moves_to_try.len();
    let move_to_do = &moves_to_try[move_index];

    (move_to_do.piece, move_to_do.move_to)
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

pub struct TheAggressiveSimpleton {}

impl Opponent for TheAggressiveSimpleton {
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

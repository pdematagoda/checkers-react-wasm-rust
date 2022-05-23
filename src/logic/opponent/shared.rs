use crate::logic::movement::move_generation::get_valid_moves;
use web_sys::console;

use crate::logic::models::Colour;
use crate::Unit;
use std::collections::HashMap;

pub fn get_active_pieces_for_side(active_pieces: &HashMap<String, Unit>, side: Colour) -> Vec<Unit> {
    let mut pieces = Vec::new();

    for (_, piece) in active_pieces {
        if piece.colour == side {
            pieces.push(*piece);
        }
    }

    pieces
}

pub fn get_pieces_with_possible_moves(
    active_pieces: &HashMap<String, Unit>,
    pieces_to_move: Vec<Unit>,
) -> Vec<Unit> {
    let mut pieces_that_can_move = pieces_to_move.clone();

    console::log_1(&format!("Starting with {} pieces", pieces_to_move.len()).into());

    pieces_that_can_move
        .retain(|&piece_to_check| get_valid_moves(&active_pieces, &piece_to_check).len() > 0);

    console::log_1(
        &format!(
            "Have {} pieces that can actually move",
            pieces_that_can_move.len()
        )
        .into(),
    );

    pieces_that_can_move
}
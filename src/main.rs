mod echiquier;
mod pieces;

use echiquier::case::Case;
use echiquier::echiquier::Echiquier;
use echiquier::piece_move::PieceMove;
use echiquier::position::Position;
use pieces::movable::Movable;
use pieces::pawn::Pawn;

fn main() {
    build_chessboard();
}

fn build_chessboard() {
    let echiquier = Echiquier::new();
    let mut position = Position::new(&echiquier);
    let e2: Case = position.echiquier.get_case(1, 4);
    let mut pawn = Pawn::new(e2, true);
    position.set_piece(*pawn);

    let one_up = PieceMove { right: 0, up: 1 };
    let e3 = position.echiquier.get_relative_case(&e2, one_up);
    let next_position = pawn.move_piece(&position, one_up, e3);

    println!("Before:\n{}", position);
    println!("\nAfter:\n{}", next_position);
}

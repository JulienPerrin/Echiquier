pub mod case;
pub mod direction;
#[expect(clippy::module_inception, reason = "echiquier struct module inside echiquier domain module")]
pub mod echiquier;
pub mod piece_move;
pub mod position;

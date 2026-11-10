#[expect(clippy::module_inception, reason = "game struct module inside game domain module")]
pub mod game;
pub mod position_change;
pub mod result;

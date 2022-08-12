use crate::game::Game;
use serde::{Deserialize, Serialize};
use shakmaty::{uci::Uci, Board, Chess, Color, Move, Role, Position};

mod capture;
pub use capture::*;

//#[derive(Serialize, Deserialize)]
//#[serde(rename_all = "kebab-case", tag = "rule")]
enum Rule {
    /// Only Pawns are allowed to capture
    MustCaptureWith {
        roles: Vec<Role>,
        color: Color,
    },
    /// Only Pawns are allowed to capture unless in check
    MustCaptureWithUnlessInCheck {
        roles: Vec<Role>,
    },

    NoCaptureWith {
        roles: Vec<Role>,
    },

    Opening {
        moves: Vec<Uci>,
    },
    PlayerSequence {
        moves: Vec<Uci>,
    },
}



pub trait GameRule {
    /// A rule that applies to an entire game
    ///
    ///
    fn is_match(&self, game: &Game, color: Color) -> bool;
}

pub trait MoveRule {
    /// A rule that can be validated on a per move/position basis
    ///
    /// pos: current position of the board
    /// mv: the last move made
    fn is_move_match(&self, pos: &Chess, mv: Move) -> bool;
}

impl<R: MoveRule> GameRule for R {
    fn is_match(&self, game: &Game, color: Color) -> bool {
        // dbg!("GameRule running: {} positions", game.iter().count());
        game.iter()
            .all(|(pos, mv)| color == pos.turn() || self.is_move_match(&pos, mv))
    }
}

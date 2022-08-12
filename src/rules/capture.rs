use super::{GameRule, MoveRule};
use shakmaty::{Chess, Color, Move, Position, Role};

/// Must capture with the specified piece type
struct MustCaptureWith {
    role: Role,
}
impl MoveRule for &MustCaptureWith {
    fn is_move_match(&self, _pos: &Chess, mv: Move) -> bool {
        !mv.is_capture() || mv.role() == self.role
    }
}

/// Must never capture the specified piece type
struct MustNotCapture {
    role: Role,
}
impl MoveRule for &MustNotCapture {
    fn is_move_match(&self, _pos: &Chess, mv: Move) -> bool {
        !mv.is_capture() || mv.capture() != Some(self.role)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::Game;
    use shakmaty::{Color::*, Role::*};

    #[test]
    fn must_capture_with_pawns_valid() {
        let game = Game::from_pgn(
            "1. d4 d5 2. e4 e6 3. exd5 Bc5 4. dxc5 Qxd5 5. Bd2 Qxg2 6. Bb5+ Kd8 7. Bg5#",
        )
        .unwrap();
        let rule = MustCaptureWith { role: Pawn };
        assert_eq!(true, game.complies(&rule, White));
    }

    #[test]
    fn must_capture_with_pawns_invalid() {
        let game = Game::from_pgn("1. d4 d5 2. e4 e6 3. exd5 Bc5 4. dxc5 Qxd5 5. c4 Qxd1+ 6. Kxd1")
            .unwrap();
        let rule = MustCaptureWith { role: Pawn };
        assert_eq!(false, game.complies(&rule, White));
    }

    #[test]
    fn must_not_capture_rook() {
        let game = Game::from_pgn("1. c4 e5 2. Qa4 Qh4 3. Qxa7 Qxh2 4. Qxb8 Qxh1 5. Qxc8+ Ke7 6. Qxb7 Qxg2 7. Qxg2")
            .unwrap();
        let rule = MustNotCapture { role: Rook };
        assert_eq!(true, game.complies(&rule, White));
        assert_eq!(false, game.complies(&rule, Black));
    }

}

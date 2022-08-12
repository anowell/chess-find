// use crate::position::Position;
use crate::rules::GameRule;
use anyhow::{bail, Result};
use pgn_reader::{BufferedReader, RawHeader, San, SanPlus, Skip, Visitor};
use shakmaty::fen::Fen;
use shakmaty::CastlingMode;
use shakmaty::{Board, Chess, Color, Move, MoveList, Position};
use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
pub struct Game {
    moves: MoveList,
}

impl Game {
    pub fn from_pgn(pgn: &str) -> Result<Game> {
        let mut reader = BufferedReader::new_cursor(pgn);
        let mut sans = SanList::default();
        reader.read_game(&mut sans)?;
        let moves = sans.to_move_list()?;
        Ok(Game {
            moves,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Chess, Move)> {
        GameIter {
            game: Chess::default(),
            moves: VecDeque::from_iter(self.moves.clone()),
        }
    }

    pub fn complies(&self, rule: impl GameRule, color: Color) -> bool {
        rule.is_match(&self, color)
    }

}

struct GameIter {
    game: Chess,
    moves: VecDeque<Move>,
}

impl Iterator for GameIter {
    type Item = (Chess, Move);

    fn next(&mut self) -> Option<Self::Item> {
        match self.moves.pop_front() {
            None => None,
            Some(m) => {
                if self.game.is_legal(&m) {
                    self.game.play_unchecked(&m);
                    Some((self.game.clone(), m))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Default)]
struct SanList(Vec<San>);
impl SanList {
    fn to_move_list(&self) -> Result<MoveList> {
        let mut pos = Chess::default();
        let mut moves = MoveList::default();
        for san in &self.0 {
            let m = san.to_move(&pos)?;
            pos.play_unchecked(&m);
            moves.push(m);
        }
        Ok(moves)
    }
}

impl Visitor for SanList {
    type Result = ();

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.0.push(san_plus.san);
    }

    fn end_game(&mut self) -> Self::Result {
        ()
    }
}

#[cfg(test)]
mod test {
    use crate::Game;

    #[test]
    fn parse_pgn() {
        let game = Game::from_pgn("1. c4 c5 2. b4").unwrap();
        assert_eq!(3, game.moves.len());
    }

    #[test]
    fn parse_pgn_illegal() {
        let res = Game::from_pgn("1. c4 c5 2. c3");
        assert!(res.is_err());
    }

}

use std::{collections::VecDeque, time::{UNIX_EPOCH, SystemTime}};

use super::*;
use rand::{rngs::StdRng, SeedableRng, seq::SliceRandom};

const MIN_SIZE: usize = 10;

pub struct Bag {
    pieces: VecDeque<PieceType>,
    rng: StdRng
}

impl Bag {
    pub fn new(random_seed: Option<u64>) -> Self {
        Self{
            pieces: VecDeque::with_capacity(MIN_SIZE*2),
            rng: StdRng::seed_from_u64(
                match random_seed {
                    Some(value) => value,
                    None => SystemTime::now().duration_since(UNIX_EPOCH)
                            .expect("Duration since UNIX_EPOCH failed").as_secs()
                }
            )
        }
    }

    pub fn get(&mut self, index: usize) -> PieceType {
        if self.pieces.len() < MIN_SIZE {
            let mut seven = vec![
                PieceType::I,
                PieceType::J,
                PieceType::L,
                PieceType::O,
                PieceType::S,
                PieceType::T,
                PieceType::Z
            ];
            seven.shuffle(&mut self.rng);
            for piece in seven {
                self.pieces.push_back(piece);
            }
        }
        if index < MIN_SIZE {
            self.pieces[index]
        } else {
            PieceType::T
        }
    }

    pub fn pop(&mut self) -> PieceType {
        let piece = self.get(0);
        self.pieces.pop_front();
        piece
    }
}
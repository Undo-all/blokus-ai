use std::intrinsics;

use player::Player;
use shape::Shape;

// TODO: Use 128-bit integers instead? I'm unsure of its effect on performance, but it would
// significantly simplify placing shapes correctly.

const WEST_MASK: u64 = 0x07FFFF7FFFF7FFFF;
const EAST_MASK: u64 = 0x0FFFFEFFFFEFFFFE;
const BOTTOM_MASK: u64 = 0x0FFFFFFFFFFFFFFF;
const ROW_MASK: u64 = 0x00000000000FFFFF;
const FIRST_MASK: u64 = 0x00000000000FFFFE;
const REST_MASK: u64 = 0x000000000007FFFF;
const HALF_MASK: u64 = 0x000000FFFFFFFFFF;

#[derive(Clone, PartialEq)]
pub struct BitBoard {
    pub blocks: [u64; 7],
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            blocks: [0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn illegal(&self, player: Player, boards: &[BitBoard; 4]) -> Self {
        let mut enemy = BitBoard::new();
        let mut turn = player;
        for _ in 0..3 {
            turn = turn.next();
            for i in 0..7 {
                enemy.blocks[i] |= boards[turn as usize].blocks[i];
            }
        }

        let enemy = enemy;

        let mut board = self.clone();

        let mut block = self.blocks[0];
        let mut flood;
        let mut prop = 0;

        // TODO: Manually unroll
        for i in 0..6 {
            flood = prop;
            flood |= (block >> 1) & WEST_MASK;
            flood |= (block << 1) & EAST_MASK;
            flood |= block >> 20;
            flood |= block << 20;
            prop = block >> 40;
            block = self.blocks[i + 1];
            flood |= (block & ROW_MASK) << 40;

            board.blocks[i] |= (flood & BOTTOM_MASK) | enemy.blocks[i];
        }

        flood = prop;
        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= block << 20;
        flood |= block >> 20;

        board.blocks[6] |= (flood & HALF_MASK) | enemy.blocks[6];
        board
    }

    pub fn corners(&self, illegal: &BitBoard) -> Self {
        let mut board = BitBoard::new();
        let mut block = self.blocks[0];
        let mut flood;
        let mut prop = 0;

        for i in 0..6 {
            flood = prop;
            flood |= (block >> 21) & WEST_MASK;
            flood |= (block << 19) & WEST_MASK;
            flood |= (block << 21) & EAST_MASK;
            flood |= (block >> 19) & EAST_MASK;
            prop = ((block >> 39) & FIRST_MASK) | ((block >> 41) & REST_MASK);
            block = self.blocks[i + 1];
            flood |= (((block >> 1) & WEST_MASK) | ((block << 1) & EAST_MASK)) << 40;
            board.blocks[i] = flood & BOTTOM_MASK & !illegal.blocks[i];
        }

        flood = prop;
        flood |= (block >> 21) & WEST_MASK;
        flood |= (block << 19) & WEST_MASK;
        flood |= (block << 21) & EAST_MASK;
        flood |= (block >> 19) & EAST_MASK;
        board.blocks[6] = flood & HALF_MASK & !illegal.blocks[6];

        board
    }

    pub fn count_tiles(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| unsafe { intrinsics::ctpop(block) as usize })
            .sum()
    }

    // TODO: Place the 5-long block correctly when its attached at the bottom or top edge of a bit
    // block (fixed by 128-bit integer use).
    pub fn place_shape(
        &self,
        shape: &Shape,
        attachment: &u8,
        at: usize,
        illegal: &BitBoard,
    ) -> Option<Self> {
        if at < (*attachment as usize) {
            return None;
        }

        let index = at - (*attachment as usize);

        if (index % 20) + (shape.width as usize) >= 20 {
            return None;
        }

        if index + (shape.height as usize) * 20 >= 400 {
            return None;
        }

        // TODO: We're copying more than we need to, earlier than we need to. Make this more
        // efficient.
        let mut copy = self.clone();

        let block = index / 60;
        let shift = index % 60;

        let shifted = shape.bits[0] << shift;
        if (shifted & illegal.blocks[block]) != 0 {
            return None;
        }

        copy.blocks[block] |= shifted & BOTTOM_MASK;

        if block == 6 {
            if (shifted & HALF_MASK) != shifted {
                None
            } else {
                Some(copy)
            }
        } else {
            let shifted = (shape.bits[0] >> (60 - shift)) | (shape.bits[1] << shift);
            if (shifted & illegal.blocks[block + 1]) != 0 {
                None
            } else {
                copy.blocks[block + 1] |= shifted & BOTTOM_MASK;
                Some(copy)
            }
        }
    }

    pub fn display(&self) {
        for block in self.blocks.iter().rev() {
            for y in (if *block == self.blocks[6] { 1 } else { 0 })..3 {
                for x in 0..20 {
                    let s = (2 - y) * 20 + x;
                    print!("{}", (block >> s) & (1 as u64));
                }

                println!();
            }
        }
    }

    pub fn is_occupied(&self, at: usize) -> bool {
        ((self.blocks[at / 60] >> (at % 60)) & 1) == 1
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|&b| b == 0)
    }

    pub fn iter(&self) -> BitIterator {
        BitIterator {
            block: 0,
            blocks: &self.blocks,
            bits: self.blocks[0],
        }
    }
}

pub struct BitIterator<'a> {
    block: u8,
    blocks: &'a [u64; 7],
    bits: u64,
}

impl<'a> BitIterator<'a> {
    fn new(board: &'a BitBoard) -> Self {
        BitIterator {
            block: 0,
            blocks: &board.blocks,
            bits: board.blocks[0],
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            let index = unsafe { intrinsics::cttz(self.bits) };

            if index == 64 {
                if self.block == 6 {
                    return None;
                } else {
                    self.block += 1;
                    self.bits = self.blocks[self.block as usize];
                }
            } else {
                self.bits &= !((1 as u64) << index);
                return Some((self.block as usize) * 60 + (index as usize));
            }
        }
    }
}

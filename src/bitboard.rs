use std::intrinsics;

use shape::Shape;

const BOTTOM_MASK: u64 = 0x00FFFFFFFFFFFFFF;
const ROW_MASK: u64 = 0x0000000000003FFF;
const EAST_MASK: u64 = 0xFFFFFBFFEFFFBFFE;
const WEST_MASK: u64 = 0xFF7FFDFFF7FFDFFF;
const FIRST_MASK: u64 = 0x0000000000003FFE;
const REST_MASK: u64 = 0x0000000000001FFF;
const HALF_MASK: u64 = 0x000000000FFFFFFF;

#[derive(Clone, PartialEq)]
pub struct BitBoard {
    blocks: [u64; 4],
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            blocks: [0, 0, 0, 0],
        }
    }

    pub fn illegal(&self, opponent: &BitBoard) -> Self {
        // let mut board = BitBoard::new();
        let mut board = self.clone();

        let block = self.blocks[0];
        let mut flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14);
        flood |= (block >> 14);
        let prop = (block >> 42);
        let block = self.blocks[1];
        flood |= (block & ROW_MASK) << 42;

        board.blocks[0] |= (flood & BOTTOM_MASK) | opponent.blocks[0];
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);
        let prop = (block >> 42);
        let block = self.blocks[2];
        flood |= (block & ROW_MASK) << 42;

        board.blocks[1] |= (flood & BOTTOM_MASK) | opponent.blocks[1];
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);
        let prop = (block >> 42);
        let block = self.blocks[3];
        flood |= (block & ROW_MASK) << 42;

        board.blocks[2] |= (flood & BOTTOM_MASK) | opponent.blocks[2];
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);

        board.blocks[3] |= (flood & HALF_MASK) | opponent.blocks[3];
        board
    }

    pub fn corners(&self, illegal: &BitBoard) -> Self {
        let mut board = BitBoard::new();

        let block = self.blocks[0];
        let mut flood = 0;

        flood |= (block >> 15) & WEST_MASK;
        flood |= (block << 13) & WEST_MASK;
        flood |= (block << 15) & EAST_MASK;
        flood |= (block >> 13) & EAST_MASK;
        let prop = ((block >> 41) & FIRST_MASK) | ((block >> 43) & REST_MASK);
        let block = self.blocks[1];
        flood |= ((block >> 1) | ((block << 1) & REST_MASK)) << 42;

        board.blocks[0] = flood & BOTTOM_MASK & !illegal.blocks[0];
        flood = 0;

        flood |= (block >> 15) & WEST_MASK;
        flood |= (block << 13) & WEST_MASK;
        flood |= (block << 15) & EAST_MASK;
        flood |= (block >> 13) & EAST_MASK;
        flood |= prop;
        let prop = ((block >> 41) & FIRST_MASK) | ((block >> 43) & REST_MASK);
        let block = self.blocks[2];
        flood |= ((block >> 1) | ((block << 1) & REST_MASK)) << 42;

        board.blocks[1] = flood & BOTTOM_MASK & !illegal.blocks[1];
        flood = 0;

        flood |= (block >> 15) & WEST_MASK;
        flood |= (block << 13) & WEST_MASK;
        flood |= (block << 15) & EAST_MASK;
        flood |= (block >> 13) & EAST_MASK;
        flood |= prop;
        let prop = ((block >> 41) & FIRST_MASK) | ((block >> 43) & REST_MASK);
        let block = self.blocks[3];
        flood |= ((block >> 1) | ((block << 1) & REST_MASK)) << 42;

        board.blocks[2] = flood & BOTTOM_MASK & !illegal.blocks[2];
        flood = 0;

        flood |= (block >> 15) & WEST_MASK;
        flood |= (block << 13) & WEST_MASK;
        flood |= (block << 15) & EAST_MASK;
        flood |= (block >> 13) & EAST_MASK;
        flood |= prop;

        board.blocks[3] = flood & HALF_MASK & !illegal.blocks[3];
        board
    }

    pub fn count_bits(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| unsafe { intrinsics::ctpop(block) as usize })
            .sum()
    }

    pub fn place_shape(&self, shape: &Shape, attachment: &u8, at: usize, illegal: &BitBoard) -> Option<Self> {
        //let index = at - (shape.attachments[attachment] as usize);
        if (at < (*attachment as usize)) {
            return None;
        }

        let index = (at as usize) - (*attachment as usize);

        if (index < 0) || ((index % 14) + (shape.width as usize) >= 14) {
            return None;
        }

        let mut copy = self.clone();

        let block = index / 56;
        let shift = index % 56;

        let shifted = shape.bits << shift;
        if (shifted & illegal.blocks[block]) != 0 {
            return None;
        }
        
        copy.blocks[block] |= shifted & BOTTOM_MASK;

        if block == 3 {
            if (shifted & HALF_MASK) != shifted {
                None
            } else {
                Some(copy)
            }
        } else {
            let shifted = shape.bits >> (56 - shift);
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
            for y in (if *block == self.blocks[3] { 2 } else { 0 })..4 {
                for x in 0..14 {
                    let s = (3 - y) * 14 + x;
                    print!("{}", (block >> s) & (1 as u64));
                }

                println!();
            }
        }
    }

    pub fn is_occupied(&self, at: usize) -> bool {
        ((self.blocks[at / 56] >> (at % 56)) & 1) == 1
    }

    pub fn iter(&self) -> BitIterator {
        BitIterator {
            block: 0,
            blocks: &self.blocks,
            bits: self.blocks[0],
        }
    }
}

pub struct BitPosition {
    pub block: u8,
    pub shift: u8,
}

pub struct BitIterator<'a> {
    block: u8,
    blocks: &'a [u64; 4],
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
                if self.block == 3 {
                    return None;
                } else {
                    self.block += 1;
                    self.bits = self.blocks[self.block as usize];
                }
            } else {
                self.bits &= !((1 as u64) << index);
                return Some((self.block as usize) * 56 + (index as usize));
            }
        }
    }
}

#![feature(nll)]
#![feature(core_intrinsics)]

use std::intrinsics;

const BOTTOM_MASK: u64 = 0x00FFFFFFFFFFFFFF;
const ROW_MASK: u64 = 0x0000000000003FFF;
const EAST_MASK: u64 = 0xFFFFFBFFEFFFBFFE;
const WEST_MASK: u64 = 0xFF7FFDFFF7FFDFFF;
const FIRST_MASK: u64 = 0x0000000000003FFE;
const REST_MASK: u64 = 0x0000000000001FFF;
const HALF_MASK: u64 = 0x000000000FFFFFFF;

#[derive(Clone)]
struct BitBoard {
    blocks: [u64; 4],
}

impl BitBoard {
    fn new() -> Self {
        BitBoard {
            blocks: [0, 0, (1 << 49) | (1 << 50), (1 << 7) | (1 << 6)],
        }
    }

    fn outline(&self) -> Self {
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

        board.blocks[0] |= flood & BOTTOM_MASK;
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);
        let prop = (block >> 42);
        let block = self.blocks[2];
        flood |= (block & ROW_MASK) << 42;
        
        board.blocks[1] |= flood & BOTTOM_MASK;
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);
        let prop = (block >> 42);
        let block = self.blocks[3];
        flood |= (block & ROW_MASK) << 42;

        board.blocks[2] |= flood & BOTTOM_MASK;
        flood = 0;

        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= (block << 14) | prop;
        flood |= (block >> 14);
        
        board.blocks[3] |= flood & HALF_MASK;
        board
    }

    fn corners(&self, illegal: &BitBoard) -> Self {
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

    fn count_bits(&self) -> usize {
        self.blocks.iter().map(|&block| unsafe { intrinsics::ctpop(block) as usize }).sum() 
    }

    fn display(&self) {
        for block in self.blocks.iter().rev() {
            for y in 0..4 {
                for x in 0..14 {
                    let s = (3-y)*14 + x;
                    print!("{}", (block >> s) & (1 as u64));
                }

                println!();
            }
        }
    }
}

struct BitPosition {
    block: usize,
    shift: usize,
}

impl BitPosition {
    fn new(block: usize, shift: usize) -> Self {
        BitPosition {
            block: block,
            shift: shift,
        }
    }
}

struct BitIterator<'a> {
    block: usize,
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
    type Item = BitPosition;

    fn next(&mut self) -> Option<BitPosition> {
        loop {
            let index = unsafe { intrinsics::cttz(self.bits) };

            if index == 64 {
                if self.block == 3 {
                    return None;
                } else {
                    self.block += 1;
                    self.bits = self.blocks[self.block];
                }
            } else {
                self.bits &= !((1 as u64) << index);
                return Some(BitPosition::new(self.block, index as usize));
            }
        }
    }
}

fn main() {
    let board = BitBoard::new();
    board.display();
    println!();
    let outline = board.outline();
    outline.display();
    println!();
    let corners = board.corners(&outline);
    corners.display();
}

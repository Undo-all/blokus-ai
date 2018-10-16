use std::collections::HashSet;
use std::mem;
use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct Shape {
    pub bits: [u64; 2],
    pub attachments: &'static [u8],
    pub width: u8,
	pub height: u8,
}

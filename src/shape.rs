use std::collections::HashSet;
use std::mem;
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Shape {
    pub bits: u64,
    pub attachments: &'static [u8],
    pub width: u8,
}

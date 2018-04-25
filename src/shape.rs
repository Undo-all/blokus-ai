use std::mem;
use std::ops::{Add, Sub};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Shape {
    pub bits: u64,
    pub attachments: &'static [usize],
}

/*
impl Shape {
    pub fn new(tiles: &[(i8, i8)]) -> Self {
        Shape::from_points(tiles.iter().map(|&(x, y)| Point { x, y }).collect())
    }

    fn from_points(mut tiles: Vec<Point>) -> Self {
        let width = tiles.iter().map(|&p| p.x).max().unwrap() as u8;
        let height = tiles.iter().map(|&p| p.y).max().unwrap() as u8;
        //let height = tiles.iter().max_by_key(|p| p.y).unwrap().y as u8;
        //let width = tiles.iter().max_by_key(|p| p.x).unwrap().x as u8;

        let bits = Shape::gen_bits(&tiles);

        let mut attachments = Vec::new();

        for &point in tiles.iter() {
            let i = point.y * 14 + point.x;

            let empty_left = (point.x == 0) || (((bits >> (i - 1)) & 1) == 0);
            let empty_right = ((bits >> (i + 1)) & 1) == 0;

            if !(empty_left || empty_right) {
                continue;
            }

            let empty_up = (point.y == 0) || (((bits >> (i - 14)) & 1) == 0);
            let empty_down = (point.y == 4) || (((bits >> (i + 14)) & 1) == 0);

            if !(empty_up || empty_down) {
                continue;
            }

            attachments.push(point);
        }

        Shape {
            tiles,
            attachments,
            bits,
            width,
            height,
        }
    }

    fn mirror_vertical(&self) -> Shape {
        let mut points = self.tiles.clone();

        for point in points.iter_mut() {
            point.y = (self.height as i8) - point.y;
        }

        Shape::from_points(points)
    }

    fn mirror_horizontal(&self) -> Shape {
        let mut points = self.tiles.clone();

        for point in points.iter_mut() {
            point.x = (self.width as i8) - point.x;
        }

        Shape::from_points(points)
    }

    fn rotate(&self) -> Shape {
        let mut points = self.tiles.clone();

        for point in points.iter_mut() {
            mem::swap(&mut point.x, &mut point.y);
        }

        Shape::from_points(points)
    }

    pub fn find_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        orientations.push(self.clone());
        orientations.push(self.mirror_horizontal());
        let vert = self.mirror_vertical();
        orientations.push(vert.clone());
        orientations.push(vert.mirror_horizontal());
        let rotated = self.rotate();
        orientations.push(rotated.clone());
        orientations.push(rotated.mirror_horizontal());
        let tmp = rotated.mirror_vertical();
        orientations.push(tmp.clone());
        orientations.push(tmp.mirror_horizontal());

        let mut unique = Vec::new();
        for orientation in orientations {
            if !unique.contains(&orientation) {
                unique.push(orientation);
            }
        }

        unique
    }

    fn gen_bits(points: &[Point]) -> u64 {
        let mut bits = 0;
        for point in points.iter() {
            bits |= ((1 as u64) << ((point.y * 14) + point.x));
        }

        bits
    }

    pub fn size(&self) -> usize {
        self.tiles.len()
    }
}
*/

use crate::pos2d::Pos;

use itertools::iproduct;
use std::convert::TryFrom;
use std::str::from_utf8;

#[derive(PartialEq, Eq)]
pub struct Map {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Map {
    pub fn new(map: &[Vec<u8>]) -> Map {
        Map {
            height: map.len(),
            width: map[0].len(),
            data: map.into_iter().cloned().flatten().collect(),
        }
    }

    pub fn new_with_size(map: &Map, cell: u8) -> Map {
        Map {
            height: map.height,
            width: map.width,
            data: vec![cell; map.data.len()],
        }
    }

    pub fn get(&self, pos: Pos) -> Option<u8> {
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        if x >= self.width{
            return None;
        }
        self.data.get(y * self.width + x).cloned()
    }

    pub fn set(&mut self, pos: Pos, cell: u8) {
        let x = usize::try_from(pos.x).expect("Map.set: illegal x coordinate");
        let y = usize::try_from(pos.y).expect("Map.set: illegal y coordinate");
        self.data[y * self.width + x] = cell
    }

    pub fn repeat_delta_from_start(
        &self,
        start: Pos,
        delta: Pos,
    ) -> impl Iterator<Item = (Pos, u8)> + '_ {
        start
            .repeat_delta_forever(delta)
            .map_while(move |pos| self.get(pos).map(move |cell| (pos, cell)))
    }

    pub fn print(&self) {
        for row in self.data.chunks(self.width) {
            println!("{}", from_utf8(row).unwrap());
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Pos, u8)> + '_ {
        self.iter_pos().zip(self.iter_cells())
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        iproduct!(0..self.height as i32, 0..self.width as i32).map(|(y, x)| Pos::new(x, y))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = u8> + '_ {
        self.data.iter().cloned()
    }
}

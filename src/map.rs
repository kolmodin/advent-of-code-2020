use crate::pos2d::Pos;

use std::convert::TryFrom;
use std::str::from_utf8;

#[derive(PartialEq, Eq)]
pub struct Map {
    vec: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(map: Vec<Vec<u8>>) -> Map {
        Map { vec: map }
    }

    pub fn new_with_size(map: &Map, cell: u8) -> Map {
        Map {
            vec: map
                .vec
                .iter()
                .map(|row| row.iter().map(|_| cell).collect())
                .collect(),
        }
    }

    pub fn get(&self, pos: Pos) -> Option<u8> {
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        self.vec.get(y).map(|row| row.get(x)).flatten().cloned()
    }

    pub fn set(&mut self, pos: Pos, cell: u8) {
        let row = self.vec.get_mut(pos.y as usize).unwrap();
        row[pos.x as usize] = cell;
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
        for row in &self.vec {
            println!("{}", from_utf8(row).unwrap());
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Pos, u8)> + '_ {
        self.iter_pos().zip(self.iter_cells())
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        let height = self.vec.len() as i32;
        let width = self.vec[0].len() as i32;
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos::new(x, y)))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = u8> + '_ {
        self.vec.iter().flat_map(|row| row.iter()).cloned()
    }
}

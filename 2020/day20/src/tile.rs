use std::convert::AsRef;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

/// Width and height of each tile.
//
// Does include the border.
pub const TILE_SIZE: usize = 10;

/// Width and height of each tile's grid.
///
/// Does not include the border.
pub const GRID_SIZE: usize = 8;

#[derive(Clone, Copy, Default)]
pub struct Tile {
    pub(crate) grid: Grid,
    pub(crate) top: u16,
    pub(crate) bottom: u16,
    pub(crate) right: u16,
    pub(crate) left: u16,
}

impl Tile {
    fn flip_horizontal(&self) -> TileRef<'_> {
        TileRef {
            tile: self,
            rotation: Rotation::None,
            flip: Flip::Horizontal,
        }
    }

    fn rotate(&self) -> TileRef<'_> {
        TileRef {
            tile: self,
            rotation: Rotation::Quarter,
            flip: Flip::None,
        }
    }

    pub fn top_edge(&self) -> u16 {
        self.top
    }
    pub fn bottom_edge(&self) -> u16 {
        self.bottom
    }
    pub fn left_edge(&self) -> u16 {
        self.left
    }
    pub fn right_edge(&self) -> u16 {
        self.right
    }

    pub fn permute(&self) -> [TileRef; 8] {
        [
            self.into(),
            self.flip_horizontal(),
            self.rotate(),
            self.rotate().flip_horizontal(),
            self.rotate().rotate(),
            self.rotate().rotate().flip_horizontal(),
            self.rotate().rotate().rotate(),
            self.rotate().rotate().rotate().flip_horizontal(),
        ]
    }

    fn compute_bitmap<I>(bits: I) -> u16
    where
        I: Iterator<Item = bool>,
    {
        let mut map = 0;
        for bit in bits {
            map <<= 1;
            map |= bit as u16;
        }
        assert!(map <= 0b1111111111);

        map
    }

    fn reverse_bitmap(mut map: u16) -> u16 {
        assert!(map <= 0b1111111111);
        let mut new = 0;

        for _ in 0..10 {
            new <<= 1;
            new |= map & 1;
            map >>= 1;
        }

        new
    }
}

impl Hash for Tile {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.grid.hash(state);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Eq for Tile {}

impl<'t> From<TileRef<'t>> for Tile {
    fn from(tile_ref: TileRef<'t>) -> Tile {
        let mut grid = tile_ref.tile.grid;

        let mut rotation = tile_ref.rotation;
        while rotation != Rotation::None {
            grid.rotate();
            rotation = rotation.prev();
        }

        if let Flip::Horizontal = tile_ref.flip {
            grid.flip_horizontal()
        }

        Tile {
            grid,
            top: tile_ref.top_edge(),
            bottom: tile_ref.bottom_edge(),
            left: tile_ref.left_edge(),
            right: tile_ref.right_edge(),
        }
    }
}

impl<'p> From<&'p [bool; TILE_SIZE * TILE_SIZE]> for Tile {
    fn from(pixels: &'p [bool; TILE_SIZE * TILE_SIZE]) -> Self {
        let mut grid = Grid::default();

        for y in 0..GRID_SIZE {
            let pixel_slice = &pixels[(y + 1) * TILE_SIZE + 1..(y + 2) * TILE_SIZE - 1];
            let grid_slice = &mut grid.0[y * GRID_SIZE..(y + 1) * GRID_SIZE];

            grid_slice.copy_from_slice(pixel_slice);
        }

        let top = Self::compute_bitmap(pixels.iter().take(10).cloned());
        let bottom = Self::compute_bitmap(pixels.iter().skip(90).cloned());
        let left = Self::compute_bitmap(pixels.iter().step_by(10).cloned());
        let right = Self::compute_bitmap(pixels.iter().skip(9).step_by(10).cloned());

        Tile {
            grid,
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Grid([bool; GRID_SIZE * GRID_SIZE]);

impl Default for Grid {
    fn default() -> Self {
        Grid([false; GRID_SIZE * GRID_SIZE])
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.0[y * GRID_SIZE + x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y * GRID_SIZE + x]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..(GRID_SIZE * GRID_SIZE) {
            write!(f, "{}", if self.0[i] { '#' } else { '.' })?;
            if (i + 1) % 10 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl AsRef<[bool; GRID_SIZE * GRID_SIZE]> for Grid {
    fn as_ref(&self) -> &[bool; GRID_SIZE * GRID_SIZE] {
        &self.0
    }
}

impl Grid {
    pub fn rotate(&mut self) {
        let old = *self;

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                self[(y, x)] = old[(GRID_SIZE - x - 1, y)];
            }
        }
    }

    pub fn flip_horizontal(&mut self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE / 2 {
                self.swap((y, x), (y, GRID_SIZE - x - 1));
            }
        }
    }

    pub fn swap(&mut self, (y, x): (usize, usize), (v, u): (usize, usize)) {
        self.0.swap(y * GRID_SIZE + x, v * GRID_SIZE + u);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Flip {
    None,
    Horizontal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rotation {
    None,
    Quarter,
    Half,
    ThreeQuarter,
}

impl Rotation {
    fn next(&self) -> Rotation {
        match self {
            Rotation::None => Rotation::Quarter,
            Rotation::Quarter => Rotation::Half,
            Rotation::Half => Rotation::ThreeQuarter,
            Rotation::ThreeQuarter => Rotation::None,
        }
    }

    fn prev(&self) -> Rotation {
        match self {
            Rotation::None => Rotation::ThreeQuarter,
            Rotation::Quarter => Rotation::None,
            Rotation::Half => Rotation::Quarter,
            Rotation::ThreeQuarter => Rotation::Half,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TileRef<'t> {
    tile: &'t Tile,
    rotation: Rotation,
    flip: Flip,
}

impl<'t> TileRef<'t> {
    pub fn rotate(&self) -> TileRef<'t> {
        assert_eq!(self.flip, Flip::None, "don't rotate flipped tiles");

        TileRef {
            tile: self.tile,
            rotation: self.rotation.next(),
            flip: Flip::None,
        }
    }

    pub fn flip_horizontal(&self) -> TileRef<'t> {
        TileRef {
            tile: self.tile,
            rotation: self.rotation,
            flip: Flip::Horizontal,
        }
    }

    pub fn top_edge(&self) -> u16 {
        match (self.rotation, self.flip) {
            (Rotation::None, Flip::None) => self.tile.top_edge(),
            (Rotation::Quarter, Flip::None) => Tile::reverse_bitmap(self.tile.left_edge()),
            (Rotation::Half, Flip::None) => Tile::reverse_bitmap(self.tile.bottom_edge()),
            (Rotation::ThreeQuarter, Flip::None) => self.tile.right_edge(),

            (Rotation::None, Flip::Horizontal) => Tile::reverse_bitmap(self.tile.top_edge()),
            (Rotation::Quarter, Flip::Horizontal) => self.tile.left_edge(),
            (Rotation::Half, Flip::Horizontal) => self.tile.bottom_edge(),
            (Rotation::ThreeQuarter, Flip::Horizontal) => {
                Tile::reverse_bitmap(self.tile.right_edge())
            }
        }
    }

    pub fn bottom_edge(&self) -> u16 {
        match (self.rotation, self.flip) {
            (Rotation::None, Flip::None) => self.tile.bottom_edge(),
            (Rotation::Quarter, Flip::None) => Tile::reverse_bitmap(self.tile.right_edge()),
            (Rotation::Half, Flip::None) => Tile::reverse_bitmap(self.tile.top_edge()),
            (Rotation::ThreeQuarter, Flip::None) => self.tile.left_edge(),

            (Rotation::None, Flip::Horizontal) => Tile::reverse_bitmap(self.tile.bottom_edge()),
            (Rotation::Quarter, Flip::Horizontal) => self.tile.right_edge(),
            (Rotation::Half, Flip::Horizontal) => self.tile.top_edge(),
            (Rotation::ThreeQuarter, Flip::Horizontal) => {
                Tile::reverse_bitmap(self.tile.left_edge())
            }
        }
    }

    pub fn left_edge(&self) -> u16 {
        match (self.rotation, self.flip) {
            (Rotation::None, Flip::None) => self.tile.left_edge(),
            (Rotation::Quarter, Flip::None) => self.tile.bottom_edge(),
            (Rotation::Half, Flip::None) => Tile::reverse_bitmap(self.tile.right_edge()),
            (Rotation::ThreeQuarter, Flip::None) => Tile::reverse_bitmap(self.tile.top_edge()),

            (Rotation::None, Flip::Horizontal) => self.tile.right_edge(),
            (Rotation::Quarter, Flip::Horizontal) => self.tile.top_edge(),
            (Rotation::Half, Flip::Horizontal) => Tile::reverse_bitmap(self.tile.left_edge()),
            (Rotation::ThreeQuarter, Flip::Horizontal) => {
                Tile::reverse_bitmap(self.tile.bottom_edge())
            }
        }
    }

    pub fn right_edge(&self) -> u16 {
        match (self.rotation, self.flip) {
            (Rotation::None, Flip::None) => self.tile.right_edge(),
            (Rotation::Quarter, Flip::None) => self.tile.top_edge(),
            (Rotation::Half, Flip::None) => Tile::reverse_bitmap(self.tile.left_edge()),
            (Rotation::ThreeQuarter, Flip::None) => Tile::reverse_bitmap(self.tile.bottom_edge()),

            (Rotation::None, Flip::Horizontal) => self.tile.left_edge(),
            (Rotation::Quarter, Flip::Horizontal) => self.tile.bottom_edge(),
            (Rotation::Half, Flip::Horizontal) => Tile::reverse_bitmap(self.tile.right_edge()),
            (Rotation::ThreeQuarter, Flip::Horizontal) => {
                Tile::reverse_bitmap(self.tile.top_edge())
            }
        }
    }
}

impl<'t> From<&'t Tile> for TileRef<'t> {
    fn from(tile: &'t Tile) -> Self {
        TileRef {
            tile,
            rotation: Rotation::None,
            flip: Flip::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_bitmap() {
        {
            assert_eq!(Tile::reverse_bitmap(0b1000000000), 0b0000000001);
        }

        {
            let tile = Tile::from(&[
                true,  true,  true,  true,  true,  true,  true,  true,  true, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, true,
                true,  true,  true,  true,  true,  true,  true,  true,  true, true,
            ]);

            assert_eq!(tile.top_edge(),    0b1111111111);
            assert_eq!(tile.right_edge(),  0b1111111111);
            assert_eq!(tile.bottom_edge(), 0b1111111111);
            assert_eq!(tile.left_edge(),   0b1111111111);
        }
        {
            let tile = Tile::from(&[
                true,  false,  true, false,  true, false,  true, false,  true, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false,
            ]);

            assert_eq!(tile.top_edge(),    0b1010101010);
            assert_eq!(tile.right_edge(),  0b0000000000);
            assert_eq!(tile.bottom_edge(), 0b0000000000);
            assert_eq!(tile.left_edge(),   0b1000000000);

            let tref = tile.flip_horizontal();
            assert_eq!(tref.top_edge(),    0b0101010101);
            assert_eq!(tref.right_edge(),  0b1000000000);
            assert_eq!(tref.bottom_edge(), 0b0000000000);
            assert_eq!(tref.left_edge(),   0b0000000000);

            let tref = tile.rotate();
            assert_eq!(tref.top_edge(),    0b0000000001);
            assert_eq!(tref.right_edge(),  0b1010101010);
            assert_eq!(tref.bottom_edge(), 0b0000000000);
            assert_eq!(tref.left_edge(),   0b0000000000);

            let tref = tile.rotate().flip_horizontal();
            assert_eq!(tref.top_edge(),    0b1000000000);
            assert_eq!(tref.right_edge(),  0b0000000000);
            assert_eq!(tref.bottom_edge(), 0b0000000000);
            assert_eq!(tref.left_edge(),   0b1010101010);

            let tref = tile.rotate().rotate();
            assert_eq!(tref.top_edge(),    0b0000000000);
            assert_eq!(tref.right_edge(),  0b0000000001);
            assert_eq!(tref.bottom_edge(), 0b0101010101);
            assert_eq!(tref.left_edge(),   0b0000000000);

            let tref = tref.flip_horizontal();
            assert_eq!(tref.top_edge(),    0b0000000000);
            assert_eq!(tref.right_edge(),  0b0000000000);
            assert_eq!(tref.bottom_edge(), 0b1010101010);
            assert_eq!(tref.left_edge(),   0b0000000001);

            let tref = tile.rotate().rotate().rotate();
            assert_eq!(tref.top_edge(),    0b0000000000);
            assert_eq!(tref.right_edge(),  0b0000000000);
            assert_eq!(tref.bottom_edge(), 0b1000000000);
            assert_eq!(tref.left_edge(),   0b0101010101);

            let tref = tref.flip_horizontal();
            assert_eq!(tref.top_edge(),    0b0000000000);
            assert_eq!(tref.right_edge(),  0b0101010101);
            assert_eq!(tref.bottom_edge(), 0b0000000001);
            assert_eq!(tref.left_edge(),   0b0000000000);
        }
    }
}

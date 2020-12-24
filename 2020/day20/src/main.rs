use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;

use crate::tile::*;

mod tile;

const PUZZLE_SIZE: usize = 12;

fn main() -> Result<()> {
    let tiles = read_input()?;

    let edges = {
        let mut edges = HashMap::<u16, HashSet<usize>>::new();
        for (i, tile) in tiles.iter() {
            for t in &tile.permute() {
                edges.entry(t.top_edge()).or_default().insert(*i);
                edges.entry(t.bottom_edge()).or_default().insert(*i);
                edges.entry(t.left_edge()).or_default().insert(*i);
                edges.entry(t.right_edge()).or_default().insert(*i);
            }
        }
        edges
    };

    let unique_edges: HashSet<u16> = edges
        .iter()
        .filter_map(|(edge, ids)| if ids.len() == 1 { Some(*edge) } else { None })
        .collect();

    // Map each tile ID to the number of unique occurrences in `edges`.
    let edge_counts: HashMap<usize, usize> = edges.iter().filter(|(_, ids)| ids.len() == 1).fold(
        HashMap::new(),
        |mut counts, (_, ids)| {
            for id in ids {
                *counts.entry(*id).or_default() += 1;
            }
            counts
        },
    );

    // Each corner tile has two unique edges. However, the vertical and
    // horizontal transformations will create an additional unique edge for
    // each, resulting in four.
    let corners = edge_counts
        .iter()
        .filter_map(|(id, count)| if *count == 4 { Some(*id) } else { None })
        .collect::<Vec<_>>();

    println!(
        "part 1: {}",
        corners.iter().map(|c| *c as u64).product::<u64>()
    );

    let mut image = assemble_image(&tiles, &edges, &unique_edges, &corners);
    remove_sea_monsters(&mut image);

    println!("part 2: {}", image.len());

    Ok(())
}

fn read_input() -> Result<HashMap<usize, Tile>> {
    const TILE_PREFIX: &str = "Tile ";
    const TILE_SUFFIX: &str = ":";

    enum State {
        ReadTileId,
        ReadGrid {
            tile_id: usize,
            y: usize,
            pixels: [bool; TILE_SIZE * TILE_SIZE],
        },
    }

    let mut tiles = HashMap::new();
    let mut state = State::ReadTileId;

    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        match state {
            State::ReadTileId => {
                if line.starts_with(TILE_PREFIX) && line.ends_with(TILE_SUFFIX) {
                    let tile_id =
                        line[TILE_PREFIX.len()..(line.len() - TILE_SUFFIX.len())].parse()?;

                    state = State::ReadGrid {
                        tile_id,
                        y: 0,
                        pixels: [false; TILE_SIZE * TILE_SIZE],
                    };
                } else {
                    panic!("unexpected input in state TileId: `{}'", line);
                }
            }

            State::ReadGrid {
                ref tile_id,
                ref mut y,
                ref mut pixels,
            } => {
                assert!(*y <= TILE_SIZE);

                if *y == TILE_SIZE {
                    tiles.insert(*tile_id, Tile::from(&*pixels));
                    state = State::ReadTileId;
                } else {
                    assert!(line.len() == 10);

                    for (x, c) in line.chars().enumerate() {
                        pixels[*y * TILE_SIZE + x] = match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("unexpected char {}", c),
                        };
                    }

                    *y += 1;
                }
            }
        }
    }

    Ok(tiles)
}

fn assemble_image(
    tiles: &HashMap<usize, Tile>,
    edges: &HashMap<u16, HashSet<usize>>,
    unique_edges: &HashSet<u16>,
    corners: &[usize],
) -> HashSet<(usize, usize)> {
    #[derive(Clone, Copy)]
    struct TileInfo<'t> {
        id: usize,
        tile: TileRef<'t>,
    }

    let mut puzzle = [None; PUZZLE_SIZE * PUZZLE_SIZE];

    // Find the top-left corner of the puzzle.
    'corners: for id in corners.iter().cloned() {
        let corner = &tiles[&id];
        for tile in corner.permute().iter().cloned() {
            let top = unique_edges.contains(&tile.top_edge());
            let bottom = unique_edges.contains(&tile.bottom_edge());
            let left = unique_edges.contains(&tile.left_edge());
            let right = unique_edges.contains(&tile.right_edge());

            if top && left && !bottom && !right {
                puzzle[0] = Some(TileInfo { id, tile });
                break 'corners;
            }
        }
    }

    assert!(puzzle[0].is_some());

    for (y, x) in (0..PUZZLE_SIZE)
        .flat_map(|y| (0..PUZZLE_SIZE).map(move |x| (y, x)))
        .skip(1)
    {
        let idx = y * PUZZLE_SIZE + x;

        let prev = if x == 0 {
            None
        } else {
            puzzle[idx - 1].as_ref()
        };

        let above = if y == 0 {
            None
        } else {
            puzzle[idx - PUZZLE_SIZE].as_ref()
        };

        let id = if let Some(info) = prev {
            *edges[&info.tile.right_edge()]
                .iter()
                .find(|id| **id != info.id)
                .unwrap()
        } else if let Some(info) = above {
            *edges[&info.tile.bottom_edge()]
                .iter()
                .find(|id| **id != info.id)
                .unwrap()
        } else {
            unreachable!("previous tile should be Some()")
        };

        for tile in tiles[&id].permute().iter().cloned() {
            if y == 0 && !unique_edges.contains(&tile.top_edge()) {
                continue;
            }

            if y > 0 && above.unwrap().tile.bottom_edge() != tile.top_edge() {
                continue;
            }

            if y != PUZZLE_SIZE - 1 && unique_edges.contains(&tile.bottom_edge()) {
                continue;
            }

            if y == PUZZLE_SIZE - 1 && !unique_edges.contains(&tile.bottom_edge()) {
                continue;
            }

            if x == 0 && !unique_edges.contains(&tile.left_edge()) {
                continue;
            }

            if x > 0 && prev.unwrap().tile.right_edge() != tile.left_edge() {
                continue;
            }

            if x != PUZZLE_SIZE - 1 && unique_edges.contains(&tile.right_edge()) {
                continue;
            }

            if x == PUZZLE_SIZE - 1 && !unique_edges.contains(&tile.right_edge()) {
                continue;
            }

            puzzle[idx] = Some(TileInfo { id, tile });
            break;
        }

        assert!(puzzle[idx].is_some(), "did not solve tile ({}, {})", y, x);
    }

    // Find each set pixel in each tile's grid and set them in the image map.
    let mut image = HashSet::new();
    for puzzle_y in 0..PUZZLE_SIZE {
        let grid_y_offset = puzzle_y * GRID_SIZE;

        for puzzle_x in 0..PUZZLE_SIZE {
            let grid_x_offset = puzzle_x * GRID_SIZE;
            let puzzle_idx = puzzle_y * PUZZLE_SIZE + puzzle_x;
            let tile: Tile = puzzle[puzzle_idx].as_ref().unwrap().tile.into();

            for grid_y in 0..GRID_SIZE {
                for grid_x in 0..GRID_SIZE {
                    if tile.grid[(grid_y, grid_x)] {
                        image.insert((grid_y_offset + grid_y, grid_x_offset + grid_x));
                    }
                }
            }
        }
    }
    image
}

fn remove_sea_monsters(image: &mut HashSet<(usize, usize)>) {
    const IMAGE_SIZE: usize = GRID_SIZE * 12;
    let mut offset_monster = [(0, 0); 15];

    for monster in sea_monsters().iter() {
        let mut found = 0;

        for y in 0..IMAGE_SIZE {
            for x in 0..IMAGE_SIZE {
                for (pt, (m_y, m_x)) in
                    Iterator::zip(offset_monster.iter_mut(), monster.iter().cloned())
                {
                    *pt = (m_y + y, m_x + x);
                }

                if offset_monster.iter().all(|pt| image.contains(pt)) {
                    found += 1;
                    for pt in offset_monster.iter() {
                        image.remove(pt);
                    }
                }
            }
        }

        if found > 0 {
            println!("found {} sea monsters", found);
            return;
        }
    }

    unreachable!("no sea monsters")
}

fn sea_monsters() -> [[(usize, usize); 15]; 8] {
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    const SEA_MONSTER: [(usize, usize); 15] = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];

    let r1 = rotate_monster(&SEA_MONSTER);
    let r2 = rotate_monster(&r1);
    let r3 = rotate_monster(&r2);

    [
        SEA_MONSTER,
        flip_monster(&SEA_MONSTER),
        r1,
        flip_monster(&r1),
        r2,
        flip_monster(&r2),
        r3,
        flip_monster(&r3),
    ]
}

fn rotate_monster(monster: &[(usize, usize); 15]) -> [(usize, usize); 15] {
    let mut rotated = [(0, 0); 15];
    let max_x = monster.iter().map(|(_, x)| x).max().unwrap();

    for (r, (y, x)) in Iterator::zip(rotated.iter_mut(), monster.iter().cloned()) {
        *r = (max_x - x, y);
    }
    rotated
}

fn flip_monster(monster: &[(usize, usize); 15]) -> [(usize, usize); 15] {
    let mut flipped = [(0, 0); 15];

    let max_x = monster.iter().map(|(_, x)| x).max().unwrap();
    for (i, (y, x)) in monster.iter().enumerate() {
        flipped[i] = (*y, max_x - *x);
    }

    flipped
}

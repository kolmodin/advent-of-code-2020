use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, PartialEq, Debug)]
struct Tile {
    id: i64,
    rows: Vec<String>,
}

impl Tile {
    fn new(id: i64, rows: Vec<String>) -> Tile {
        Tile { id, rows }
    }
    fn top(&self) -> String {
        self.rows.first().cloned().unwrap()
    }
    fn bottom(&self) -> String {
        self.rows.last().cloned().unwrap()
    }
    fn left(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.chars().next().unwrap())
            .collect()
    }
    fn right(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.chars().last().unwrap())
            .collect()
    }
    fn print(&self) {
        println!("Tile: {}", self.id);
        for r in &self.rows {
            println!("{}", r);
        }
        println!();
    }

    fn rotations(&self) -> Vec<Tile> {
        let res: Vec<Tile> = rotations(&self.rows)
            .into_iter()
            .map(|rows| Tile { id: self.id, rows })
            .collect();

        let check: Vec<String> = res.iter().map(|t| t.top()).unique().collect();
        assert_eq!(check.len(), 8);
        res
    }
}

fn rotations(rows: &[String]) -> Vec<Vec<String>> {
    let mut res = vec![];
    let mut t: Vec<String> = rows.iter().cloned().collect();

    // top, left, bottom, right
    for _ in 0..4 {
        let next = rotate_left(&t);
        res.push(t);
        t = next;
    }

    t = hflip(&t);
    // top, left, bottom, right
    for _ in 0..4 {
        let next = rotate_left(&t);
        res.push(t);
        t = next;
    }

    res
}

fn hflip(rows: &[String]) -> Vec<String> {
    rotate_right(&vflip(&rotate_left(rows)))
}

fn vflip(rows: &[String]) -> Vec<String> {
    rows.iter().rev().cloned().collect()
}

fn rotate_right(rows: &[String]) -> Vec<String> {
    rotate_left(&rotate_left(&rotate_left(rows)))
}

fn rotate_left(rows: &[String]) -> Vec<String> {
    let m: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let mut new: Vec<Vec<char>> = Vec::new();
    for _new_height_idx in 0..m[0].len() {
        let mut new_row = Vec::new();
        new_row.resize(m.len(), ' ');
        new.push(new_row);
    }
    for old_y in 0..m.len() {
        let new_x = old_y;
        for old_x in 0..m[0].len() {
            let new_y = m.len() - 1 - old_x;
            new[new_y][new_x] = m[old_y][old_x];
        }
    }
    new.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn parse_tile(inp: &str) -> Tile {
    let mut rows: Vec<String> = inp.lines().map(|s| s.to_string()).collect();
    let top = rows[0].clone();
    rows.remove(0);
    // Tile 2111:
    let id = &top[5..9].parse::<i64>().unwrap();
    Tile::new(*id, rows)
}

type ID = i64;

fn flatten_tiles(tiles: &[Vec<Tile>]) -> Vec<String> {
    let mut img: Vec<String> = vec![];
    let tile_len = tiles[0][0].rows.len();
    for tile_row in tiles {
        for y_in_tile in 1..tile_row[0].rows.len() - 1 {
            let mut img_row: String = String::new();
            for t in tile_row {
                img_row += &t.rows[y_in_tile][1..tile_len - 1];
            }
            img.push(img_row);
        }
    }
    img
}

fn main() {
    let contents = fs::read_to_string("inputs/day20.txt").expect("Something went wrong reading the file");

    let tiles: Vec<Tile> = contents.split("\n\n").map(parse_tile).collect();

    let mut counter: HashMap<ID, i64> = HashMap::new();

    let tiles_p1: Vec<(Tile, Vec<Tile>)> = tiles // .
        .iter()
        .map(|t| (t.clone(), t.rotations()))
        .collect();
    for (t1, rots1) in &tiles_p1 {
        for r1 in &rots1[0..4] {
            for (t2, rots2) in &tiles_p1 {
                if t1.id == t2.id {
                    continue;
                }
                if rots2.iter().any(|r2| r2.top() == r1.top()) {
                    let e = counter.entry(t1.id).or_insert(0);
                    *e += 1;
                }
            }
        }
    }

    let all_corner_ids: Vec<ID> = counter
        .iter()
        .filter_map(|(k, v)| match *v {
            2 => Some(k),
            _ => None,
        })
        .cloned()
        .collect();
    println!("Part 1: {}", all_corner_ids.iter().product::<i64>());

    let id_map: HashMap<ID, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();

    let top_map = {
        let mut hm: HashMap<String, Vec<Tile>> = HashMap::new();
        for t in &tiles {
            for t_rot in t.rotations() {
                let e = hm.entry(t_rot.top()).or_insert(vec![]);
                e.push(t_rot);
            }
        }
        hm
    };

    println!("all corner ids: {:?}", &all_corner_ids);
    let mut ids: Vec<ID> = tiles.iter().map(|t| t.id).collect();
    ids.sort_unstable();

    // Find a corner tile that already has the right orientation.
    // Top and left sides are unique.
    let corner_tile = all_corner_ids
        .iter()
        .filter(|id| {
            top_map[&id_map[*id].top()].len() == 1 && // .
            top_map[&id_map[*id].left()].len() == 1
        })
        .map(|id| id_map[id].clone())
        .next()
        .unwrap();
    println!("selected start:");
    corner_tile.print();

    let mut seen: HashSet<ID> = HashSet::new();
    let mut puzzle: Vec<Vec<Tile>> = vec![];

    let mut to_left = corner_tile.clone();

    let mut current_row: Vec<Tile> = vec![];
    current_row.push(to_left.clone());
    seen.insert(to_left.id);
    loop {
        let to_right_opt = tiles
            .iter()
            .filter(|t| !seen.contains(&t.id))
            .flat_map(|t| t.rotations())
            .filter(|t| to_left.right() == t.left())
            .next();

        if let Some(to_right) = to_right_opt {
            current_row.push(to_right.clone());
            seen.insert(to_right.id);
            to_left = to_right;
            continue;
        }

        // end of row
        // find new start of the next row.
        // looking for top_left.bottom() == new.top()
        println!("tiles in finished row: {}", current_row.len());
        let top_left = current_row.first().cloned().unwrap();
        let rows_in_tile = top_left.rows.len();
        println!(
            "ids: {:?}",
            current_row.iter().map(|t| t.id).collect::<Vec<_>>()
        );

        puzzle.push(current_row);

        let bottom_opt = tiles
            .iter()
            .filter(|t| !seen.contains(&t.id))
            .flat_map(|t| t.rotations())
            .filter(|t| top_left.bottom() == t.top())
            .next();

        if let Some(bottom) = bottom_opt {
            seen.insert(bottom.id);
            current_row = vec![bottom.clone()];
            to_left = bottom;
            continue;
        }

        assert_eq!(seen.len(), tiles.len());
        break;
    }

    println!("\n");
    let flattened = flatten_tiles(&puzzle);

    let monster_str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #";

    let monster_pos: Vec<_> = monster_str
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    println!("{:?}", monster_pos);
    let dots = flattened
        .iter()
        .flat_map(|row| row.chars().filter(|c| *c == '#'))
        .count();
    println!("dots {}", dots);

    for (i, rot_map) in rotations(&flattened).iter().enumerate() {
        let mut monster_count = 0;
        for (startx, starty) in iproduct!(0..flattened[0].len(), 0..flattened.len()) {
            let found_one = monster_pos.iter().all(|(x, y)| {
                rot_map
                    .get(starty + y)
                    .map(|row| row.chars().nth(startx + x))
                    .flatten()
                    == Some('#')
            });
            if found_one {
                monster_count += 1;
            }
        }
        if monster_count > 0 {
            println!("\nwinning map: {}", i);
            for ln in rot_map {
                println!("{}", ln);
            }
            println!("monster count {}", monster_count);
            println!("Part 2: {}", dots - monster_pos.len() * monster_count);
        }
    }

    println!("done");
}

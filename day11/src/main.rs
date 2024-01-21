use itertools::Itertools;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Tile {
    Space,
    Galaxy,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("invalid tile type!"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Space => ".",
            Self::Galaxy => "#",
        };
        write!(f, "{repr}")
    }
}

struct GalaxyCoords {
    row: i64,
    col: i64,
}

impl GalaxyCoords {
    fn find_distance(&self, other: &Self) -> usize {
        usize::try_from((self.row - other.row).abs() + (self.col - other.col).abs()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpandedTile {
    actual_width: usize,
    actual_height: usize,
    tile: Tile,
}

impl fmt::Display for ExpandedTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.actual_width, self.actual_height)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpaceMap(Vec<Vec<Tile>>);

impl SpaceMap {
    fn expand_space(self, expansion_factor: usize) -> ExpandedMap {
        let num_cols = self.0[0].len();
        let expanded_map_vec: Vec<Vec<ExpandedTile>> = self
            .0
            .iter()
            .map(|row| {
                let actual_height = if row.iter().all(|t| *t == Tile::Space) {
                    expansion_factor
                } else {
                    1
                };
                let expanded_row: Vec<ExpandedTile> = row
                    .iter()
                    .map(|t| ExpandedTile {
                        actual_height,
                        actual_width: 1,
                        tile: *t,
                    })
                    .collect();
                expanded_row
            })
            .collect();
        let mut expanded_map = ExpandedMap(expanded_map_vec);
        for i in 0..num_cols {
            if self.0.iter().all(|row| row[i] == Tile::Space) {
                expanded_map
                    .0
                    .iter_mut()
                    .for_each(|row| row[i].actual_width = expansion_factor);
            }
        }
        expanded_map
    }
}

impl fmt::Display for SpaceMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for tile in row {
                write!(f, "{tile} ")?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
struct ExpandedMap(Vec<Vec<ExpandedTile>>);
impl fmt::Display for ExpandedMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for tile in row {
                write!(f, "{tile} ")?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("{}", run("input.txt", 1000000))
}

fn create_space_map(filename: &str) -> SpaceMap {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut space_map = SpaceMap(vec![]);
    for line in reader.lines() {
        let line = line.unwrap();
        space_map
            .0
            .push(line.chars().map(|c| Tile::from_char(c)).collect());
    }
    space_map
}

fn run(filename: &str, expansion_factor: usize) -> usize {
    let space_map = create_space_map(filename).expand_space(expansion_factor);

    collect_galaxies(space_map)
        .iter()
        .combinations(2)
        .map(|galaxy_pair| galaxy_pair[0].find_distance(&galaxy_pair[1]))
        .sum()
}

fn collect_galaxies(space_map: ExpandedMap) -> Vec<GalaxyCoords> {
    let mut cursor_row = 0;
    let mut cursor_col;
    let mut galaxies: Vec<GalaxyCoords> = vec![];
    for row in space_map.0.iter() {
        cursor_col = 0;
        for tile in row.iter() {
            if tile.tile == Tile::Galaxy {
                galaxies.push(GalaxyCoords {
                    row: cursor_row,
                    col: cursor_col as i64,
                });
            }
            cursor_col += tile.actual_width as i64;
        }
        cursor_row += row[0].actual_height as i64;
    }
    galaxies
}

#[test]
fn sample_test_factor10() {
    assert_eq!(run("sample_input.txt", 10), 1030);
}

#[test]
fn sample_test_factor100() {
    assert_eq!(run("sample_input.txt", 100), 8410);
}

#[test]
fn test_create_space_map() {
    let space_map = create_space_map("sample_input.txt").expand_space(10);
    println!("{space_map}");
    assert_eq!(space_map.0[0][2].actual_width, 10);
    assert_eq!(space_map.0[0][5].actual_width, 10);
    assert_eq!(space_map.0[0][8].actual_width, 10);
    assert_eq!(space_map.0[3][0].actual_height, 10);
    assert_eq!(space_map.0[3][0].actual_width, 1);
}

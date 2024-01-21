use itertools::Itertools;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
struct SpaceMap(Vec<Vec<Tile>>);

impl SpaceMap {
    fn expand_space(self) -> Self {
        let mut temp = SpaceMap(vec![]);
        let num_cols = self.0[0].len();
        for i in 0..self.0.len() {
            if self.0[i].iter().all(|t| *t == Tile::Space) {
                temp.0.push(self.0[i].clone());
            }
            temp.0.push(self.0[i].clone());
        }
        let mut new_space = temp.clone();
        let mut num_col_expansions = 0;
        for i in 0..num_cols {
            if temp.0.iter().all(|row| row[i] == Tile::Space) {
                for row in new_space.0.iter_mut() {
                    row.insert(i + num_col_expansions, Tile::Space);
                }
                num_col_expansions += 1;
            }
        }
        new_space
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

fn main() {
    println!("{}", run("input.txt"))
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

fn run(filename: &str) -> usize {
    let space_map = create_space_map(filename).expand_space();

    let mut galaxies: Vec<GalaxyCoords> = vec![];
    for (i, row) in space_map.0.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Galaxy {
                galaxies.push(GalaxyCoords {
                    row: i as i64,
                    col: j as i64,
                });
            }
        }
    }
    galaxies
        .iter()
        .combinations(2)
        .map(|galaxy_pair| galaxy_pair[0].find_distance(&galaxy_pair[1]))
        .sum()
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 374);
}

#[test]
fn test_create_space_map() {
    let space_map = create_space_map("sample_input.txt").expand_space();
    println!("{space_map}");
    let expanded = create_space_map("sample_expansion.txt");
    println!("{expanded}");
    assert_eq!(space_map.0.len(), expanded.0.len());
    assert_eq!(space_map.0[0].len(), expanded.0[0].len());
    for (i, (l, r)) in space_map.0.iter().zip(expanded.0.iter()).enumerate() {
        println!("{i}");
        assert_eq!(l, r);
    }
    assert_eq!(space_map, expanded);
}

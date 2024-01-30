use std::{
    convert::From,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

type MirrorMatrix = Vec<Vec<Tile>>;

trait Energize {
    fn energize(&mut self, heading: Heading, start_point: Point);
    fn energize_north(&mut self, start_point: Point);
    fn energize_south(&mut self, start_point: Point);
    fn energize_east(&mut self, start_point: Point);
    fn energize_west(&mut self, start_point: Point);
}

impl Energize for MirrorMatrix {
    fn energize(&mut self, heading: Heading, start_point: Point) {
        let Point {
            row: start_row,
            col: start_col,
        } = start_point;
        println!("Heading {heading:?} from {start_row}, {start_col}");
        match heading {
            Heading::North => {
                for i in (0..=start_row).rev() {
                    let tile = &mut self[i][start_col];
                    // if tile.energised
                    //     && tile.mirror.is_some()
                    //     && matches!(tile.mirror.unwrap(), Mirror::SplitterH)
                    // {
                    //     break;
                    // }
                    tile.energised = true;
                    if tile.mirror.is_some() {
                        let new_start = Point {
                            row: i,
                            col: start_col,
                        };
                        match tile.mirror.unwrap() {
                            Mirror::BackSlash => {
                                self.energize_west(new_start);
                                break;
                            }
                            Mirror::ForwardSlash => {
                                self.energize_east(new_start);
                                break;
                            }
                            Mirror::SplitterH => {
                                self.energize_east(new_start);
                                self.energize_west(new_start);
                                break;
                            }
                            Mirror::SplitterV => continue,
                        }
                    }
                }
            }
            Heading::South => {
                let num_rows = self.len();
                for i in start_row..num_rows {
                    let tile = &mut self[i][start_col];
                    // if tile.energised
                    //     && tile.mirror.is_some()
                    //     && matches!(tile.mirror.unwrap(), Mirror::SplitterH)
                    // {
                    //     break;
                    // }
                    tile.energised = true;
                    if tile.mirror.is_some() {
                        let new_start = Point {
                            row: i,
                            col: start_col,
                        };
                        match tile.mirror.unwrap() {
                            Mirror::BackSlash => {
                                self.energize_east(new_start);
                                break;
                            }
                            Mirror::ForwardSlash => {
                                self.energize_west(new_start);
                                break;
                            }
                            Mirror::SplitterH => {
                                self.energize_east(new_start);
                                self.energize_west(new_start);
                                break;
                            }
                            Mirror::SplitterV => continue,
                        }
                    }
                }
            }
            Heading::East => {
                let num_cols = self[0].len();
                for j in start_col..num_cols {
                    let tile = &mut self[start_row][j];
                    // if tile.energised
                    //     && tile.mirror.is_some()
                    //     && matches!(tile.mirror.unwrap(), Mirror::SplitterV)
                    // {
                    //     break;
                    // }
                    tile.energised = true;
                    if tile.mirror.is_some() {
                        let new_start = Point {
                            row: start_row,
                            col: j,
                        };
                        match tile.mirror.unwrap() {
                            Mirror::BackSlash => {
                                self.energize_south(new_start);
                                break;
                            }
                            Mirror::ForwardSlash => {
                                self.energize_north(new_start);
                                break;
                            }
                            Mirror::SplitterH => {
                                continue;
                            }
                            Mirror::SplitterV => {
                                self.energize_north(new_start);
                                self.energize_south(new_start);
                                break;
                            }
                        }
                    }
                }
            }
            Heading::West => {
                let num_cols = self[0].len();
                for j in (start_col..num_cols).rev() {
                    let tile = &mut self[start_row][j];
                    // if tile.energised
                    //     && tile.mirror.is_some()
                    //     && matches!(tile.mirror.unwrap(), Mirror::SplitterV)
                    // {
                    //     break;
                    // }
                    tile.energised = true;
                    if tile.mirror.is_some() {
                        let new_start = Point {
                            row: start_row,
                            col: j,
                        };
                        match tile.mirror.unwrap() {
                            Mirror::BackSlash => {
                                self.energize_north(new_start);
                                break;
                            }
                            Mirror::ForwardSlash => {
                                self.energize_south(new_start);
                                break;
                            }
                            Mirror::SplitterH => {
                                continue;
                            }
                            Mirror::SplitterV => {
                                self.energize_north(new_start);
                                self.energize_south(new_start);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn energize_north(&mut self, start_point: Point) {
        let new_start = Point {
            row: if start_point.row > 0 {
                start_point.row - 1
            } else {
                return;
            },
            col: start_point.col,
        };
        self.energize(Heading::North, start_point);
    }
    fn energize_south(&mut self, start_point: Point) {
        let new_start = Point {
            row: if start_point.row < (self.len() - 1) {
                start_point.row + 1
            } else {
                return;
            },
            col: start_point.col,
        };
        self.energize(Heading::South, start_point);
    }
    fn energize_east(&mut self, start_point: Point) {
        let new_start = Point {
            row: start_point.row,
            col: if start_point.col < (self[0].len() - 1) {
                start_point.col + 1
            } else {
                return;
            },
        };
        self.energize(Heading::East, start_point);
    }
    fn energize_west(&mut self, start_point: Point) {
        let new_start = Point {
            row: start_point.row,
            col: if start_point.col > 0 {
                start_point.col - 1
            } else {
                return;
            },
        };
        self.energize(Heading::West, start_point);
    }
}

#[derive(Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Mirror {
    BackSlash,
    ForwardSlash,
    SplitterH,
    SplitterV,
}

impl From<char> for Mirror {
    fn from(c: char) -> Self {
        match c {
            '\\' => Self::BackSlash,
            '/' => Self::ForwardSlash,
            '-' => Self::SplitterH,
            '|' => Self::SplitterV,
            _ => panic!("Unexpected mirror shape {c} encountered"),
        }
    }
}

#[derive(Debug)]
struct Tile {
    mirror: Option<Mirror>,
    energised: bool,
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.energised {
            write!(fmt, "#")
        } else {
            write!(fmt, ".")
        }
    }
}

fn main() {
    println!("{}", run("input.txt"));
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut mirrors: MirrorMatrix = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        mirrors.push(
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Tile {
                            mirror: None,
                            energised: false,
                        }
                    } else {
                        Tile {
                            mirror: Some(Mirror::from(c)),
                            energised: false,
                        }
                    }
                })
                .collect(),
        );
    }
    mirrors.energize(Heading::East, Point { row: 0, col: 0 });
    for row in mirrors.iter() {
        row.iter().for_each(|t| print!("{t}"));
        println!("");
    }
    mirrors
        .iter()
        .map(|row| row.iter().filter(|tile| tile.energised).count())
        .sum()
}

#[test]
fn test_sample() {
    assert_eq!(46, run("sample_input.txt"));
}

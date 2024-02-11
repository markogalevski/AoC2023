use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    cost: usize,
    position: Position,
    num_steps: usize,
}

impl Ord for State {
    /* inverted logic here ensures we have a max heap and not a min heap */
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    location: Point,
    cost: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    heading: Direction,
    location: Point,
}

impl Position {
    fn rotate_and_step(&self, towards: Rotation) -> Self {
        let heading = self.heading.rotate(towards);
        Self {
            location: self.location + heading.offset(),
            heading,
        }
    }
    fn step(&self) -> Self {
        Self {
            location: self.location + self.heading.offset(),
            heading: self.heading,
        }
    }
}

enum Rotation {
    Cw,
    Ccw,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, rot: Rotation) -> Self {
        match rot {
            Rotation::Cw => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
            Rotation::Ccw => match self {
                Self::North => Self::West,
                Self::West => Self::South,
                Self::South => Self::East,
                Self::East => Self::North,
            },
        }
    }

    fn offset(&self) -> Point {
        match self {
            Self::North => Point { row: -1, col: 0 },
            Self::East => Point { row: 0, col: 1 },
            Self::South => Point { row: 1, col: 0 },
            Self::West => Point { row: 0, col: -1 },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: i64,
    col: i64,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

fn main() {
    println!("{}", run("input.txt", 4, 10));
}

fn run(filename: &str, min_steps: usize, max_steps: usize) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut edge_list: Vec<Vec<Node>> = vec![];
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        edge_list.push(
            line.chars()
                .enumerate()
                .map(|(j, c)| Node {
                    location: Point {
                        row: i as i64,
                        col: j as i64,
                    },
                    cost: c.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<Node>>(),
        );
    }
    let start = edge_list[0][0];
    let end = edge_list[edge_list.len() - 1][edge_list[0].len() - 1];
    dijkstra_binary_heap(edge_list, &start, &end, min_steps, max_steps).unwrap()
}

fn dijkstra_binary_heap(
    edge_list: Vec<Vec<Node>>,
    start: &Node,
    goal: &Node,
    min_steps: usize,
    max_steps: usize,
) -> Option<usize> {
    let edge_map: HashMap<Point, usize> = HashMap::from_iter(
        edge_list
            .iter()
            .flatten()
            .map(|node| (node.location, node.cost))
            .collect::<Vec<_>>()
            .into_iter(),
    );
    let mut distances: HashMap<Point, usize> = HashMap::from_iter(
        edge_list
            .iter()
            .flatten()
            .map(|node| (node.location, usize::MAX))
            .collect::<Vec<(Point, usize)>>()
            .into_iter(),
    );
    let num_rows = edge_list.len();
    let num_cols = edge_list[0].len();
    let mut heap = BinaryHeap::new();
    let mut seen: HashSet<(Position, usize)> = HashSet::new();

    /* Modifications to a normal heap based dijkstra are as follows:
      A node (i.e. state) is normally the cost we paid to enter it and any associated data.
      Here, however, we can enter a node from various directions, and having had taken a various
      number of steps. Each of these permutations is a actually a different state to be in, as our
      choice are influenced by these parameters.

      Therefore, we need to track both the cost of the node, it's location, where we came from, and how
    many steps we took before we got to it.

    This heap based dijkstra uses a "seen"/visited list as opposed to an explicit iteration over all nodes.
    */
    distances.insert(start.location, 0);
    heap.push(State {
        cost: 0,
        position: Position {
            heading: Direction::East,
            location: start.location,
        },
        num_steps: 0,
    });
    heap.push(State {
        cost: 0,
        position: Position {
            heading: Direction::South,
            location: start.location,
        },
        num_steps: 0,
    });

    while let Some(State {
        cost,
        position,
        num_steps,
    }) = heap.pop()
    {
        /* Did we find the goal? Cool, let's return the cost */
        if position.location == goal.location {
            return Some(cost);
        }

        /* if the cost is already worse than our minimum value found, why bother caclulating further? */
        if cost > distances[&position.location] {
            continue;
        }

        /* Insert API returns true only if the element was brand new. So here we skip if the value was already present*/
        if !seen.insert((position, num_steps)) {
            continue;
        }

        /* The following situation calculates up to 3 possible new states that have to be added to the heap for processing, depending
        on the num_steps constraints provided as input */
        if num_steps >= min_steps {
            let left = position.rotate_and_step(Rotation::Ccw);
            if is_location_valid(&left.location, num_rows as i64, num_cols as i64) {
                heap.push(State {
                    cost: cost + edge_map.get(&left.location).unwrap_or(&0),
                    position: left,
                    num_steps: 1,
                });
            }
            let right = position.rotate_and_step(Rotation::Cw);
            if is_location_valid(&right.location, num_rows as i64, num_cols as i64) {
                heap.push(State {
                    cost: cost + edge_map.get(&right.location).unwrap_or(&0),
                    position: right,
                    num_steps: 1,
                });
            }
        }

        let forward = position.step();
        if num_steps < max_steps
            && is_location_valid(&forward.location, num_rows as i64, num_cols as i64)
        {
            heap.push(State {
                cost: cost + edge_map.get(&forward.location).unwrap_or(&0),
                position: forward,
                num_steps: num_steps + 1,
            });
        }
    }
    None
}

fn is_location_valid(loc: &Point, num_rows: i64, num_cols: i64) -> bool {
    if loc.row < 0 || loc.col < 0 {
        return false;
    }
    loc.row < num_rows && loc.col < num_cols
}

#[test]
fn test_sample1() {
    assert_eq!(run("sample_input.txt", 0, 3), 102);
}

#[test]
fn test_sample2() {
    assert_eq!(run("sample_input.txt", 4, 10), 94);
}

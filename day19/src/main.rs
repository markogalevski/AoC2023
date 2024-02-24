use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    convert::From,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

fn main() -> Result<()> {
    println!("{}", run("input.txt")?);
    Ok(())
}

fn run(filename: &str) -> Result<i64> {
    let workflows = read_workflows(filename)?;

    let urnode = Node {
        component: Component {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        target: Target::Workflow("in".to_owned()),
    };
    let mut queue = vec![urnode];
    let mut accepted: Vec<Component> = vec![];
    while let Some(node) = queue.pop() {
        match node.target {
            Target::Rejected => (),
            Target::Accepted => accepted.push(node.component),
            Target::Workflow(ref key) => {
                let workflow = workflows.get(key).unwrap();
                workflow.evaluate(node, &mut queue);
            }
        }
    }
    Ok(accepted.iter().map(|acc| acc.combinations()).sum())
}

fn read_workflows(filename: &str) -> Result<HashMap<String, Workflow>> {
    let file = File::open(filename).with_context(|| "Unable to open file {filename}")?;
    let reader = BufReader::new(file);
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let line_preprocessed = line.replace('{', " ").replace('}', "");
        let split: Vec<&str> = line_preprocessed.split_whitespace().collect();
        let key = split[0].to_owned();
        let workflow = Workflow::from(split[1]);
        workflows.insert(key, workflow);
    }
    Ok(workflows)
}

#[derive(Hash, Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate(&self, node: Node, global_q: &mut Vec<Node>) {
        let mut component = node.component.clone();
        for rule in self.rules.iter() {
            if let Some(ref comparison) = rule.comparison {
                //do comparison
                let (passing, blocked) = comparison.compare_and_split(component);
                global_q.push(Node {
                    component: passing,
                    target: rule.target.clone(),
                });
                component = blocked;
            } else {
                let node = Node {
                    component: component.clone(),
                    target: rule.target.clone(),
                };
                global_q.push(node);
            }
        }
    }
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        Self {
            rules: s.split(',').map(Rule::from).collect(),
        }
    }
}

#[derive(Hash, Debug)]
struct Rule {
    comparison: Option<Comparison>,
    target: Target,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if s.contains(':') {
            let split: Vec<&str> = s.split(':').collect();
            let s = split[0];
            let key = split[1];
            Self {
                comparison: Some(Comparison::from(s)),
                target: Target::from(key),
            }
        } else {
            Self {
                comparison: None,
                target: Target::from(s),
            }
        }
    }
}

#[derive(Hash, Debug)]
struct Comparison {
    characteristic: Xmas,
    op: Op,
    val: i64,
}

impl Comparison {
    fn compare_and_split(&self, component: Component) -> (Component, Component) {
        let comp_val = match self.characteristic {
            Xmas::X => &component.x,
            Xmas::M => &component.m,
            Xmas::A => &component.a,
            Xmas::S => &component.s,
        };
        let (passing_range, block_range) = match self.op {
            Op::Lt => {
                let passing_range = comp_val.start..self.val;
                let block_range = (self.val)..comp_val.end;
                (passing_range, block_range)
            }
            Op::Gt => {
                let block_range = comp_val.start..(self.val + 1);
                let passing_range = (self.val + 1)..comp_val.end;
                (passing_range, block_range)
            }
        };
        match self.characteristic {
            Xmas::X => (
                Component {
                    x: passing_range,
                    ..component.clone()
                },
                Component {
                    x: block_range,
                    ..component.clone()
                },
            ),
            Xmas::M => (
                Component {
                    m: passing_range,
                    ..component.clone()
                },
                Component {
                    m: block_range,
                    ..component.clone()
                },
            ),
            Xmas::A => (
                Component {
                    a: passing_range,
                    ..component.clone()
                },
                Component {
                    a: block_range,
                    ..component.clone()
                },
            ),
            Xmas::S => (
                Component {
                    s: passing_range,
                    ..component.clone()
                },
                Component {
                    s: block_range,
                    ..component.clone()
                },
            ),
        }
    }
}

impl From<&str> for Comparison {
    fn from(s: &str) -> Comparison {
        let split: Vec<&str> = s.split(':').collect();
        let s = split[0];
        let mut chars = s.chars();
        Self {
            characteristic: Xmas::from(chars.next().unwrap()),
            op: Op::from(chars.next().unwrap()),
            val: chars.collect::<String>().parse().unwrap(),
        }
    }
}

#[derive(Hash, Debug)]
enum Xmas {
    X,
    M,
    A,
    S,
}

impl From<char> for Xmas {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("invalid xmas character"),
        }
    }
}

#[derive(Hash, Debug)]
enum Op {
    Lt,
    Gt,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Lt,
            '>' => Self::Gt,
            _ => panic!("Invalid comparison character"),
        }
    }
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
enum Target {
    Rejected,
    Accepted,
    Workflow(String),
}

impl From<&str> for Target {
    fn from(s: &str) -> Self {
        match s {
            "R" => Self::Rejected,
            "A" => Self::Accepted,
            &_ => Self::Workflow(s.to_owned()),
        }
    }
}

#[derive(Clone)]
struct Node {
    component: Component,
    target: Target,
}

#[derive(Clone)]
struct Component {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}

impl Component {
    fn combinations(&self) -> i64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }
}

#[test]
fn sample() {
    assert_eq!(run("sample_input.txt").unwrap(), 167409079868000);
}

use std::collections::VecDeque;
use std::rc::Rc;
use std::{
    cell::RefCell,
    collections::HashMap,
    convert::From,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
fn main() -> Result<()> {
    println!("{}", run("input.txt")?);
    Ok(())
}

fn run(filename: &str) -> Result<usize> {
    let (mut graph, mut lookup) = build_graph_and_lookup(filename)?;
    let mut high_signals: usize = 0;
    let mut low_signals: usize = 0;

    graph
        .nodes
        .iter_mut()
        .for_each(|node| node.borrow_mut().reset_levels());
    for _ in 1..=1000 {
        press_button(&mut lookup, &graph, &mut high_signals, &mut low_signals);
    }
    println!("High: {high_signals}");
    println!("Low: {low_signals}");

    Ok(high_signals * low_signals)
}

fn press_button(
    lookup: &mut HashMap<String, WrappedNode>,
    graph: &Graph,
    high_signals: &mut usize,
    low_signals: &mut usize,
) {
    let mut signal_queue: VecDeque<Signal> = VecDeque::from([Signal {
        level: State::Low,
        to: "broadcaster".to_owned(),
        from: "button".to_owned(),
    }]);

    *low_signals += 1;
    while let Some(signal) = signal_queue.pop_front() {
        if signal.to == "rx" || signal.to == "output" {
            continue;
        }
        let recipient = lookup.get(&signal.to).unwrap();
        if let Some(idx) = graph
            .nodes
            .iter()
            .position(|n| *n.borrow() == *recipient.as_ref().borrow())
        {
            let adj_list = &graph.adj_lists[idx];
            let from = signal.to.clone();
            if let Some(new_level) = recipient.borrow_mut().generate_output_state(&signal) {
                for adj in adj_list {
                    if new_level == State::High {
                        *high_signals += 1;
                    } else {
                        *low_signals += 1;
                    }

                    let new_signal = Signal {
                        from: from.clone(),
                        level: new_level,
                        to: adj.clone(),
                    };

                    signal_queue.push_back(new_signal);
                }
            }
        }
    }
}

fn build_graph_and_lookup(
    filename: &str,
) -> Result<(Graph, HashMap<String, WrappedNode>), anyhow::Error> {
    let file = File::open(filename).with_context(|| "Unable to open file {filename}")?;
    let reader = BufReader::new(file);
    let mut graph = Graph::new();
    let mut node_lookup: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    graph.nodes.push(Rc::new(RefCell::new(Node::Button)));
    graph.adj_lists.push(vec!["broadcaster".to_owned()]);
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split("->").collect();
        let node = Rc::new(RefCell::new(Node::from(split[0].trim())));
        node_lookup.insert(node.borrow().get_name(), node.clone());
        graph.nodes.push(node);
        graph
            .adj_lists
            .push(split[1].split(',').map(|s| s.trim().to_owned()).collect());
    }
    for (adj_list, source_node) in graph.adj_lists.iter().zip(graph.nodes.iter()) {
        for adj in adj_list.iter() {
            if let Some(target_node) = node_lookup.get_mut(adj) {
                let target_node = &mut *target_node.borrow_mut();
                if let &mut Node::Conjunct(ref mut c) = target_node {
                    let source_name = &*source_node.borrow().get_name();
                    c.froms.push(source_name.to_owned());
                    c.inputs.push(State::Low);
                }
            }
        }
    }
    Ok((graph, node_lookup))
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
enum State {
    High,
    #[default]
    Low,
}

impl State {
    fn toggle(&mut self) {
        if *self == Self::High {
            *self = Self::Low;
        } else {
            *self = Self::High;
        }
    }
}

type WrappedNode = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    nodes: Vec<WrappedNode>,
    adj_lists: Vec<Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            adj_lists: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Button, //Not generateable
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunct(Conjunct),
}

impl Node {
    fn get_name(&self) -> String {
        match self {
            Self::Button => "button".to_owned(),
            Self::Broadcaster => "broadcaster".to_owned(),
            Self::FlipFlop(FlipFlop { key, .. }) | Self::Conjunct(Conjunct { key, .. }) => {
                key.clone()
            }
        }
    }
    fn reset_levels(&mut self) {
        match self {
            Node::Button | Node::Broadcaster => (),
            Node::FlipFlop(f) => f.reset_levels(),
            Node::Conjunct(c) => c.reset_levels(),
        }
    }

    fn generate_output_state(&mut self, input_signal: &Signal) -> Option<State> {
        match self {
            Self::Button | Self::Broadcaster => Some(State::Low),
            Self::FlipFlop(f) => f.process(input_signal),
            Self::Conjunct(c) => c.process(input_signal),
        }
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split("->").collect();
        let key = split[0].to_owned().replace(['%', '&'], "");
        if split[0] == "broadcaster" {
            Self::Broadcaster
        } else if s.starts_with('%') {
            Self::FlipFlop(FlipFlop {
                key,
                ..Default::default()
            })
        } else if s.starts_with('&') {
            Self::Conjunct(Conjunct {
                key,
                ..Default::default()
            })
        } else {
            panic!("AAAA Don't try to add output or rx here");
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FlipFlop {
    key: String,
    state: State,
}

impl FlipFlop {
    fn process(&mut self, input_signal: &Signal) -> Option<State> {
        if input_signal.level == State::Low {
            self.state.toggle();
            Some(self.state)
        } else {
            None
        }
    }
    fn reset_levels(&mut self) {
        self.state = State::Low;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Conjunct {
    key: String,
    inputs: Vec<State>,
    froms: Vec<String>,
}

impl Conjunct {
    fn process(&mut self, input_signal: &Signal) -> Option<State> {
        if let Some(index) = self.froms.iter().position(|f| *f == input_signal.from) {
            self.inputs[index] = input_signal.level;
        } else {
            self.froms.push(input_signal.from.clone());
            self.inputs.push(input_signal.level)
        }

        if self.inputs.iter().all(|input| *input == State::High) {
            Some(State::Low)
        } else {
            Some(State::High)
        }
    }

    fn reset_levels(&mut self) {
        self.inputs.iter_mut().for_each(|input| *input = State::Low);
    }
}

#[derive(Debug, Clone, Default)]
struct Signal {
    level: State,
    to: String,
    from: String,
}

#[test]
fn sample1() {
    assert_eq!(run("sample_input1.txt").unwrap(), 32000000);
}
#[test]
fn sample2() {
    assert_eq!(run("sample_input2.txt").unwrap(), 11687500);
}

#[test]
fn conjunction_behaviour_inverter() {
    let mut conj = Conjunct::default();
    let input_signal = Signal {
        level: State::Low,
        ..Default::default()
    };
    assert_eq!(conj.process(&input_signal), Some(State::High));
    assert_eq!(conj.process(&input_signal), Some(State::High));
    let input_signal = Signal {
        level: State::High,
        ..Default::default()
    };
    assert_eq!(conj.process(&input_signal), Some(State::Low));
    assert_eq!(conj.process(&input_signal), Some(State::Low));
}

#[test]
fn conjunction_behaviour_and() {
    let mut conj = Conjunct::default();
    let mut input_signal1 = Signal {
        from: "A".to_owned(),
        level: State::Low,
        ..Default::default()
    };
    let mut input_signal2 = Signal {
        from: "B".to_owned(),
        level: State::Low,
        ..Default::default()
    };

    assert_eq!(conj.process(&input_signal1), Some(State::High));
    assert_eq!(conj.process(&input_signal2), Some(State::High));
    input_signal1.level = State::High;
    assert_eq!(conj.process(&input_signal1), Some(State::High));
    assert_eq!(conj.process(&input_signal2), Some(State::High));
    input_signal2.level = State::High;
    assert_eq!(conj.process(&input_signal1), Some(State::High));
    assert_eq!(conj.process(&input_signal2), Some(State::Low));
    input_signal2.level = State::Low;
    assert_eq!(conj.process(&input_signal1), Some(State::Low));
    assert_eq!(conj.process(&input_signal2), Some(State::High));
    input_signal1.level = State::Low;
    assert_eq!(conj.process(&input_signal1), Some(State::High));
    assert_eq!(conj.process(&input_signal2), Some(State::High));
}

#[test]
fn flipflop_behaviour() {
    let mut ff = FlipFlop::default();
    assert_eq!(ff.state, State::Low);
    let input_signal = Signal {
        level: State::Low,
        ..Default::default()
    };
    ff.process(&input_signal);
    assert_eq!(ff.state, State::High);
    ff.process(&input_signal);
    assert_eq!(ff.state, State::Low);
    let input_signal = Signal {
        level: State::High,
        ..Default::default()
    };
    ff.process(&input_signal);
    assert_eq!(ff.state, State::Low);
    ff.process(&input_signal);
    assert_eq!(ff.state, State::Low);
}

#[test]
fn with_input() {
    assert_eq!(run("input.txt").unwrap(), 834323022)
}

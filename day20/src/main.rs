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
    let (graph, mut lookup) = build_graph_and_lookup(filename)?;
    let mut high_signals: usize = 0;
    let mut low_signals: usize = 0;

    for _ in 1..=1000 {
        let high_at_start = high_signals;
        let low_at_start = low_signals;
        press_button(&mut lookup, &graph, &mut high_signals, &mut low_signals);

        println!("generated highs: {}", high_signals - high_at_start);
        println!("generated lows: {}", low_signals - low_at_start);
        println!("-------");
    }
    println!("High: {high_signals}");
    println!("Low: {low_signals}");

    Ok(high_signals * low_signals)
}

fn press_button(
    lookup: &mut HashMap<String, Rc<RefCell<Node>>>,
    graph: &Graph,
    high_signals: &mut usize,
    low_signals: &mut usize,
) {
    let mut signal_queue = vec![Signal {
        level: State::Low,
        to: "broadcaster".to_owned(),
        from: "button".to_owned(),
    }];
    *low_signals += 1;
    while let Some(signal) = signal_queue.pop() {
        let Some(recipient) = lookup
            .get(&signal.to) else {
            println!("just tried to send a signal to {}, adding to lookup" , signal.to);
            lookup.insert(signal.to.clone(), Rc::new(RefCell::new(Node::Sink(Sink {key: signal.to.clone()}))));
            continue;
        };
        if let Some(idx) = graph
            .nodes
            .iter()
            .position(|n| *n.borrow() == *recipient.as_ref().borrow())
        {
            let adj_list = &graph.adj_lists[idx];
            let from = signal.to.clone();
            if let Some(new_signal_level) = recipient.borrow_mut().generate_output_state(signal) {
                for adj in adj_list {
                    if new_signal_level == State::High {
                        *high_signals += 1;
                    } else {
                        *low_signals += 1;
                    }
                    let new_signal = Signal {
                        from: from.clone(),
                        level: new_signal_level,
                        to: adj.clone(),
                    };
                    println!("pushing {new_signal:?} to the queue");
                    signal_queue.push(new_signal);
                }
            }
        }
    }
}

fn build_graph_and_lookup(
    filename: &str,
) -> Result<(Graph, HashMap<String, Rc<RefCell<Node>>>), anyhow::Error> {
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
    // println!("printing graph...");
    // for (node, adj) in graph.nodes.iter().zip(graph.adj_lists.iter()) {
    //     println!("{node:?}: {adj:?}");
    // }
    // println!("printing lookup...");
    // for (key, value) in node_lookup.iter() {
    //     println!("{key}: {value:?}");
    // }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
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
    Sink(Sink),
}

impl Node {
    fn get_name(&self) -> String {
        match self {
            Self::Button => "button".to_owned(),
            Self::Broadcaster => "broadcaster".to_owned(),
            Self::FlipFlop(FlipFlop { key, .. }) | Self::Conjunct(Conjunct { key, .. }) => {
                key.clone()
            }
            Self::Sink(Sink { key }) => key.clone(),
        }
    }

    fn generate_output_state(&mut self, input_signal: Signal) -> Option<State> {
        match self {
            Self::Button | Self::Broadcaster => Some(State::Low),
            Self::FlipFlop(f) => f.process(input_signal),
            Self::Conjunct(c) => c.process(input_signal),
            Self::Sink(_) => None,
        }
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split("->").collect();
        let key = split[0].to_owned().replace('%', "").replace('&', "");
        if split[0] == "broadcaster" {
            Self::Broadcaster
        } else if s.starts_with("%") {
            Self::FlipFlop(FlipFlop {
                key,
                ..Default::default()
            })
        } else if s.starts_with("&") {
            Self::Conjunct(Conjunct {
                key,
                ..Default::default()
            })
        } else {
            Self::Sink(Sink { key })
        }
    }
}
#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Sink {
    key: String,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FlipFlop {
    key: String,
    state: State,
}

impl FlipFlop {
    fn process(&mut self, input_signal: Signal) -> Option<State> {
        if input_signal.level == State::Low {
            self.state.toggle();
            Some(self.state)
        } else {
            None
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Conjunct {
    key: String,
    first_input: State,
    second_input: State,
    first_from: Option<String>,
    second_from: Option<String>,
}

impl Conjunct {
    fn process(&mut self, input_signal: Signal) -> Option<State> {
        if let Some(ref first_from) = self.first_from {
            if input_signal.from == *first_from {
                self.first_input = input_signal.level;
            }
        } else {
            self.first_input = input_signal.level;
            self.first_from = Some(input_signal.from.clone());
        }

        if let Some(ref second_from) = self.second_from {
            if input_signal.from == *second_from {
                self.second_input = input_signal.level;
            }
        } else {
            self.second_input = input_signal.level;
        }

        self.calc_and()
    }

    fn calc_and(&self) -> Option<State> {
        if self.first_input == State::High && self.second_input == State::High {
            Some(State::Low)
        } else {
            Some(State::High)
        }
    }
}

#[derive(Debug, Clone)]
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

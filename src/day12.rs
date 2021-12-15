use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::ops::Deref;
use std::rc::Rc;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};

use crate::day12::State::{Clean, Revisit, Start};

#[derive(Debug)]
struct EdgeDefinition<'a> {
    labels: [&'a str; 2],
}

struct Edge<'a> {
    nodes: [Rc<Node<'a>>; 2],
}

struct Node<'a> {
    label: &'a str,
    edges: RefCell<Vec<Rc<Edge<'a>>>>,
}

impl Debug for Node<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label)
    }
}

impl<'a> Edge<'a> {
    fn other(self: Rc<Self>, node: Rc<Node<'a>>) -> Rc<Node<'a>> {
        if self.nodes[0].deref() == node.deref() {
            self.nodes[1].clone()
        } else {
            self.nodes[0].clone()
        }
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Node<'_> {}

impl Node<'_> {
    fn is_large(&self) -> bool {
        self.label.as_bytes()[0].is_ascii_uppercase()
    }
}

struct Graph<'a> {
    start: Rc<Node<'a>>,
    end: Rc<Node<'a>>,
}

fn line(input: &str) -> IResult<&str, EdgeDefinition> {
    map(tuple((
        alpha1,
        tag("-"),
        alpha1
    )), |(l1, _, l2)| EdgeDefinition { labels: [l1, l2] })(input)
}

fn parse(input: &str) -> IResult<&str, Graph> {
    map(many1(terminated(line, tag("\n"))), |lines| {
        let mut nodes = HashMap::new();
        for line in lines {
            let mut node1 = nodes.entry(line.labels[0]).or_insert_with(|| Rc::new(Node {
                label: line.labels[0],
                edges: RefCell::new(Vec::new()),
            })).clone();
            let mut node2 = nodes.entry(line.labels[1]).or_insert_with(|| Rc::new(Node {
                label: line.labels[1],
                edges: RefCell::new(Vec::new()),
            })).clone();
            let edge = Rc::new(Edge {
                nodes: [node1.clone(), node2.clone()]
            });
            node1.edges.borrow_mut().push(edge.clone());
            node2.edges.borrow_mut().push(edge);
        }
        Graph {
            start: nodes.get("start").unwrap().clone(),
            end: nodes.get("end").unwrap().clone(),
        }
    })(input)
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    Start,
    Clean,
    Revisit
}

fn depth_search_end<'long, 'temp>(cursor: Rc<Node<'long>>, path: &'temp mut Vec<Rc<Node<'long>>>, state: State) -> usize {
    if state != Start && cursor.label == "start" {
        0
    } else if cursor.label == "end" {
        1
    } else {
        let mut next_state = if state == Start {
            Clean
        } else if let Some(prev) = path.iter().find(|p| (*p).clone() == cursor.clone()) {
            if !prev.is_large() {
                if state == Revisit {
                    return 0;
                } else {
                    Revisit
                }
            } else {
                state
            }
        } else {
            state
        };
        path.push(cursor.clone());
        let mut count = 0;
        for edge in cursor.edges.borrow().iter() {
            count += depth_search_end(edge.clone().other(cursor.clone()), path, next_state)
        }

        path.pop();
        count
    }
}

pub fn solve(input: &str) -> String {
    let (_, graph) = parse(input).unwrap();
    let mut temp = vec![];
    let path_count = depth_search_end(graph.start.clone(), &mut temp, Start);
    format!("Got graph with path count {}", path_count)
}
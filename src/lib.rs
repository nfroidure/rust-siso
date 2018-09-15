extern crate regex;
mod route;

use regex::Regex;
use std::collections::HashMap;
use std::error;
use std::fmt;
use route::Route;

pub struct Siso<T: Clone + PartialEq> {
    route: Route<T>,
}

impl<T: Clone + PartialEq> Siso<T> {
    pub fn register(&mut self, path_nodes: &[PathNode], value: T) -> Result<(), SisoError> {
        if path_nodes.is_empty() {
            panic!("Need at least one node for registering a value...");
        }
        self.route.register(path_nodes, value)
    }

    pub fn find(&self, path_nodes: &[PathNode]) -> Result<SisoResult<T>, SisoError> {
        if path_nodes.is_empty() {
            panic!("Need at least one node for finding a value...");
        }
        self.route.find(path_nodes)
    }

    pub fn new() -> Siso<T> {
        Siso {
            route: Route::new(PathNode::PathPart(String::from("[root]")), None),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SisoResult<T: Clone + PartialEq> {
    nodes: Vec<&'static str>,
    values: HashMap<String, bool>,
    handler: T,
}

#[derive(Debug, Clone)]
pub struct SisoError {
    code: String,
}

impl fmt::Display for SisoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path already declared")
    }
}

impl error::Error for SisoError {
    fn description(&self) -> &str {
        "path already declared"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct PathPattern {
    name: String,
    pattern: Regex,
    // TODO: Add a way to parse the node and cast its value
}

#[derive(Debug, Clone)]
pub enum PathNode {
    PathPart(String),
    PathPattern(PathPattern),
}

// TODO: Try #[derive(PartialEq)] once the lib is feature complete
impl PartialEq for PathNode {
    fn eq(&self, other: &PathNode) -> bool {
        match self {
            PathNode::PathPart(self_part) => match other {
                PathNode::PathPart(other_part) => self_part == other_part,
                _ => false,
            },
            PathNode::PathPattern(self_pattern) => match other {
                PathNode::PathPattern(other_pattern) => self_pattern.name == other_pattern.name,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_node_comparison_works() {
        assert_eq!(
            PathNode::PathPart(String::from("foo")),
            PathNode::PathPart(String::from("foo"))
        );
        assert_ne!(
            PathNode::PathPart(String::from("foo")),
            PathNode::PathPart(String::from("bar"))
        );
        // TODO: Also compare PathPattern
    }
    #[test]
    fn router_works() {
        // TODO: Also test PathPattern nodes
        let route1 = vec![
            PathNode::PathPart(String::from("foo")),
            PathNode::PathPart(String::from("bar")),
        ];
        let route2 = vec![
            PathNode::PathPart(String::from("foo")),
            PathNode::PathPart(String::from("lol")),
        ];
        let route3 = vec![
            PathNode::PathPattern(PathPattern {
                name: String::from("foo"),
                pattern: Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap(),
            }),
            PathNode::PathPart(String::from("lol")),
        ];
        let no_route = vec![
            PathNode::PathPart(String::from("no")),
            PathNode::PathPart(String::from("thing")),
        ];
        let mut router = Siso::new();

        assert_eq!(
            router
                .register(route1.as_slice(), String::from("test1"))
                .unwrap(),
            ()
        );
        assert_eq!(
            router
                .register(route2.as_slice(), String::from("test2"))
                .unwrap(),
            ()
        );
        assert_eq!(
            router
                .register(route3.as_slice(), String::from("test3"))
                .unwrap(),
            ()
        );

        assert_eq!(
            router.find(route1.as_slice()).unwrap(),
            SisoResult {
                nodes: Vec::new(),
                values: HashMap::new(),
                handler: String::from("test1"),
            }
        );
        router.find(no_route.as_slice()).expect_err("E_NOT_FOUND");
        router
            .register(route1.as_slice(), String::from("test3"))
            .expect_err("E_NOT_FOUND");
    }
}

extern crate regex;
mod route;

use regex::Regex;
use route::FindContext;
use route::Route;
use std::collections::HashMap;
use std::error;
use std::fmt;

pub struct Siso<V: Clone + PartialEq> {
    route: Route<V>,
}

impl<V: Clone + PartialEq> Siso<V> {
    pub fn register(
        &mut self,
        path_nodes: &[Box<MatchPathNode>],
        value: V,
    ) -> Result<(), SisoError> {
        if path_nodes.is_empty() {
            panic!("Need at least one node for registering a value...");
        }
        self.route.register(path_nodes, value)
    }

    pub fn find(&self, path_nodes: &[String]) -> Result<SisoResult<V>, SisoError> {
        if path_nodes.is_empty() {
            panic!("Need at least one node for finding a value...");
        }
        self.route.find(path_nodes, FindContext::new())
    }

    pub fn new() -> Siso<V> {
        Siso {
            route: Route::new(None, None),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SisoResult<V: Clone + PartialEq> {
    nodes: Vec<&'static str>,
    values: HashMap<String, bool>,
    handler: V,
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

pub trait MatchPathNode {
    fn path_node_match(&self, &str) -> bool {
        false
    }
    fn to_uid(&self) -> String;
    fn box_clone(&self) -> Box<MatchPathNode>;
}

impl Clone for Box<MatchPathNode> {
    fn clone(&self) -> Box<MatchPathNode> {
        self.box_clone()
    }
}

impl MatchPathNode for String {
    fn path_node_match(&self, other: &str) -> bool {
        return *self == *other;
    }
    fn to_uid(&self) -> String {
        self.clone()
    }
    fn box_clone(&self) -> Box<MatchPathNode> {
        Box::new((*self).clone())
    }
}

#[derive(Debug, Clone)]
pub struct PathPattern {
    name: String,
    pattern: Regex,
    // TODO: Add a way to parse the node and cast its value
}

impl PartialEq for PathPattern {
    fn eq(&self, ref other: &PathPattern) -> bool {
        self.name == other.name
    }
}

impl MatchPathNode for PathPattern {
    fn path_node_match(&self, other: &str) -> bool {
        return self.pattern.is_match(other);
    }
    fn to_uid(&self) -> String {
        self.name.clone()
    }
    fn box_clone(&self) -> Box<MatchPathNode> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_node_comparison_works() {
        assert!(String::from("foo").path_node_match("foo"));
        assert!(!String::from("foo").path_node_match("bar"));
        assert!(
            PathPattern {
                name: String::from("node1"),
                pattern: Regex::new(r"^test\d+$").unwrap(),
            }.path_node_match("test5")
        );
    }

    #[test]
    fn router_works() {
        // TODO: Also test PathPattern nodes
        let route1 : Vec<Box<MatchPathNode>> = vec![Box::new(String::from("foo")), Box::new(String::from("bar"))];
        let route2 : Vec<Box<MatchPathNode>> = vec![Box::new(String::from("foo")), Box::new(String::from("lol"))];
        let route3 : Vec<Box<MatchPathNode>> = vec![
            Box::new(PathPattern {
                name: String::from("node1"),
                pattern: Regex::new(r"^test\d+$").unwrap(),
            }),
            Box::new(PathPattern {
                name: String::from("node2"),
                pattern: Regex::new(r"^test\d+$").unwrap(),
            }),
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
            router.find(vec!["foo".to_string(), "bar".to_string()].as_slice()).unwrap(),
            SisoResult {
                nodes: Vec::new(),
                values: HashMap::new(),
                handler: String::from("test1"),
            }
        );
        router.find(vec!["no".to_string(), "thing".to_string()].as_slice()).expect_err("E_NOT_FOUND");
        router
            .register(route1.as_slice(), String::from("test3"))
            .expect_err("E_NOT_FOUND");
    }
}

extern crate regex;
mod route;

use regex::Regex;
use std::collections::HashMap;
use std::error;
use std::fmt;
use route::Route;
use route::FindContext;

pub struct Siso<T: MatchPathNode + Clone, V: Clone + PartialEq> {
    route: Route<T, V>,
}

impl<T: MatchPathNode + Clone, V: Clone + PartialEq> Siso<T, V> {
    pub fn register(&mut self, path_nodes: &[T], value: V) -> Result<(), SisoError> {
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

    pub fn new() -> Siso<T, V> {
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
   fn path_node_equals(&self, &Self) -> bool;
}

impl MatchPathNode for String {
    fn path_node_match(&self, other: &str) -> bool {
        return *self == *other;
    }
   fn path_node_equals(&self, other: &String) -> bool {
       self == other
   }
}

#[derive(Debug)]
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
   fn path_node_equals(&self, other: &PathPattern) -> bool {
       *self == *other
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_node_comparison_works() {
        assert!(
            String::from("foo").path_node_match("foo")
        );
        assert!(
            !String::from("foo").path_node_match("bar")
        );
        // TODO: Also compare PathPattern
    }
    #[test]
    fn router_works() {
        // TODO: Also test PathPattern nodes
        let route1 = vec![
            String::from("foo"),
            String::from("bar"),
        ];
        let route2 = vec![
            String::from("foo"),
            String::from("lol"),
        ];
        let route3 = vec![
            PathPattern {
                name: String::from("node1"),
                pattern: Regex::new(r"^test\d+$").unwrap(),
            },
            PathPattern {
                name: String::from("node2"),
                pattern: Regex::new(r"^test\d+$").unwrap(),
            },
        ];
        let no_route = vec![
            String::from("no"),
            String::from("thing"),
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
        /*assert_eq!(
            router
                .register(route3.as_slice(), String::from("test3"))
                .unwrap(),
            ()
        );*/

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

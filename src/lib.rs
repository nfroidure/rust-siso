extern crate regex;
use regex::Regex;
use std::error;
use std::fmt;

pub struct Siso<T: Clone + PartialEq> {
    route: Route<T>,
}

impl<T: Clone + PartialEq> Siso<T> {
    pub fn register(
        &mut self,
        path_nodes: &[PathNode],
        value: RouteValue<T>,
    ) -> Result<(), SisoError> {
        if path_nodes.is_empty() {
            panic!("Need at least one node for registering a value...");
        }
        self.route.register(path_nodes, value)
    }

    pub fn find(&self, path_nodes: &[PathNode]) -> Result<RouteValue<T>, SisoError> {
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

#[derive(Debug)]
struct Route<T: Clone + PartialEq> {
    node: PathNode,
    routes: Vec<Route<T>>,
    value: RouteValue<T>,
}

impl<T: Clone + PartialEq> Route<T> {
    fn register(&mut self, path_nodes: &[PathNode], value: RouteValue<T>) -> Result<(), SisoError> {
        let path_node = &path_nodes[0];
        let is_leaf_path_node = path_nodes.len() == 1;
        let index = self.routes.iter()
            .position(|a_route| *path_node == a_route.node)
            .unwrap_or_else(|| {
                let route = Route::new(path_node.clone(), None);
                self.routes.push(route);
                self.routes.len() - 1
            });

        if is_leaf_path_node {
            match self.routes[index].value {
                RouteValue::Value(_) => Err(SisoError {
                    code: "E_VALUE_EXISTS".to_string(),
                }),
                RouteValue::None => {
                    self.routes[index].value = value;
                    Ok(())
                }
            }
        } else {
            self.routes[index].register(&path_nodes[1..], value)
        }
    }
    fn find(&self, path_nodes: &[PathNode]) -> Result<RouteValue<T>, SisoError> {
        for a_route in &self.routes {
            if path_nodes[0] == a_route.node {
                if path_nodes.len() == 1 {
                    return Ok(a_route.value.clone());
                } else {
                    return a_route.find(&path_nodes[1..]);
                }
            }
        }
        Err(SisoError {
            code: "E_NOT_FOUND".to_string(),
        })
    }

    fn new(node: PathNode, value: Option<RouteValue<T>>) -> Route<T> {
        Route {
            routes: Vec::new(),
            node,
            value: value.unwrap_or(RouteValue::None),
        }
    }
}

// TODO: Allow generic value instead of strings only
#[derive(Debug, Clone)]
pub enum RouteValue<T: Clone + PartialEq> {
    Value(T),
    None,
}

// Here we intentionnally want Node != None since no value for the path x
// is not the same concept that no value for the path y
impl<T: Clone + PartialEq> PartialEq for RouteValue<T> {
    fn eq(&self, other: &RouteValue<T>) -> bool {
        match self {
            RouteValue::Value(self_value) => match other {
                RouteValue::Value(other_value) => self_value == other_value,
                RouteValue::None => false,
            },
            RouteValue::None => false,
        }
    }
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
                pattern: Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
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
                .register(route1.as_slice(), RouteValue::Value(String::from("test1")))
                .unwrap(),
            ()
        );
        assert_eq!(
            router
                .register(route2.as_slice(), RouteValue::Value(String::from("test2")))
                .unwrap(),
            ()
        );
        assert_eq!(
            router
                .register(route3.as_slice(), RouteValue::Value(String::from("test3")))
                .unwrap(),
            ()
        );

        assert_eq!(
            router.find(route1.as_slice()).unwrap(),
            RouteValue::Value(String::from("test1"))
        );
        router.find(no_route.as_slice()).expect_err("E_NOT_FOUND");
        router
            .register(route1.as_slice(), RouteValue::Value(String::from("test3")))
            .expect_err("E_NOT_FOUND");
    }
}

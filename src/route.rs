use super::*;

#[derive(Debug)]
pub struct Route<T: Clone + PartialEq> {
    node: PathNode,
    routes: Vec<Route<T>>,
    value: Option<T>,
}

impl<T: Clone + PartialEq> Route<T> {
    pub fn register(&mut self, path_nodes: &[PathNode], value: T) -> Result<(), SisoError> {
        let path_node = &path_nodes[0];
        let is_leaf_path_node = path_nodes.len() == 1;
        let index = self
            .routes
            .iter()
            .position(|a_route| *path_node == a_route.node)
            .unwrap_or_else(|| {
                let route = Route::new(path_node.clone(), None);
                self.routes.push(route);
                self.routes.len() - 1
            });

        if is_leaf_path_node {
            match self.routes[index].value {
                Some(_) => Err(SisoError {
                    code: "E_VALUE_EXISTS".to_string(),
                }),
                None => {
                    self.routes[index].value = Some(value);
                    Ok(())
                }
            }
        } else {
            self.routes[index].register(&path_nodes[1..], value)
        }
    }
    pub fn find(&self, path_nodes: &[PathNode]) -> Result<SisoResult<T>, SisoError> {
        for a_route in &self.routes {
            if path_nodes[0] == a_route.node {
                if path_nodes.len() == 1 {
                    return match &a_route.value {
                        Some(value) => Ok(SisoResult {
                            nodes: Vec::new(),
                            values: HashMap::new(),
                            handler: value.clone(),
                        }),
                        None => Err(SisoError {
                            code: "E_NOT_FOUND".to_string(),
                        }),
                    };
                } else {
                    return a_route.find(&path_nodes[1..]);
                }
            }
        }
        Err(SisoError {
            code: "E_NOT_FOUND".to_string(),
        })
    }

    pub fn new(node: PathNode, value: Option<T>) -> Route<T> {
        Route {
            routes: Vec::new(),
            node,
            value: value,
        }
    }
}


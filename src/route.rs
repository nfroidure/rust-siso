use super::*;

pub struct Route<V: Clone + PartialEq> {
    node: Option<Box<MatchPathNode>>,
    routes: Vec<Route<V>>,
    value: Option<V>,
}

impl<V: Clone + PartialEq> Route<V> {
    pub fn register(
        &mut self,
        path_nodes: &[Box<MatchPathNode>],
        value: V,
    ) -> Result<(), SisoError> {
        let path_node = &path_nodes[0];
        let is_leaf_path_node = path_nodes.len() == 1;
        let index = self
            .routes
            .iter()
            .position(|a_route| a_route.node.as_ref().unwrap().to_uid() == path_node.to_uid())
            .unwrap_or_else(|| {
                let route = Route::new(Some(path_node.clone()), None);
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
    pub fn find(
        &self,
        path_nodes: &[String],
        context: FindContext,
    ) -> Result<SisoResult<V>, SisoError> {
        for a_route in &self.routes {
            if a_route
                .node
                .as_ref()
                .unwrap()
                .path_node_match(&path_nodes[0][..])
            {
                if path_nodes.len() == 1 {
                    return match &a_route.value {
                        Some(value) => Ok(SisoResult {
                            nodes: context.nodes,
                            values: context.values,
                            handler: value.clone(),
                        }),
                        None => Err(SisoError {
                            code: "E_NOT_FOUND".to_string(),
                        }),
                    };
                } else {
                    return a_route.find(&path_nodes[1..], context);
                }
            }
        }
        Err(SisoError {
            code: "E_NOT_FOUND".to_string(),
        })
    }

    pub fn new(node: Option<Box<MatchPathNode>>, value: Option<V>) -> Route<V> {
        Route {
            routes: Vec::new(),
            node,
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct FindContext {
    nodes: Vec<&'static str>,
    values: HashMap<String, bool>,
}

impl FindContext {
    pub fn new() -> FindContext {
        FindContext {
            nodes: Vec::new(),
            values: HashMap::new(),
        }
    }
}

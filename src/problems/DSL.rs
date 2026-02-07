// https://exercism.org/tracks/rust/exercises/dot-dsl

macro_rules! impl_attrs {
    ($struct:ident) => {
        impl $struct {
            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs.extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                self
            }

            pub fn attr(&self, attr_name: &str) -> Option<&str> {
                self.attrs.get(attr_name).map(|s| s.as_str())
            }
        }
    };
}

pub mod graph {
    use std::collections::HashMap;
    use crate::graph::attr::Attr;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attr,
    }
    impl_attrs!(Graph);

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node_name)
        }
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;
            use crate::graph::attr::Attr;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: Attr,
            }
            impl_attrs!(Edge);

            impl Edge {
                pub fn new<'a>(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            use crate::graph::attr::Attr;

            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: Attr,
            }
            impl_attrs!(Node);

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }
    }

    pub mod attr {
        use std::collections::HashMap;

        pub type Attr = HashMap<String, String>;
    }
}

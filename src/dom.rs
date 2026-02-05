use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Document,
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split_whitespace().collect(),
            None => HashSet::new(),
        }
    }
}

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type,
        }
    }

    pub fn text(data: String) -> Node {
        Node::new(NodeType::Text(data), Vec::new())
    }

    pub fn element(name: String, attrs: HashMap<String, String>, children: Vec<Node>) -> Node {
        Node::new(
            NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
            children,
        )
    }
}

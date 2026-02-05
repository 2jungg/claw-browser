use crate::dom::{Node, NodeType};
use scraper::Html;
use std::collections::HashMap;

pub fn parse(html: &str) -> Node {
    let document = Html::parse_document(html);
    convert_ego_node(document.tree.root())
}

fn convert_ego_node(node_ref: ego_tree::NodeRef<scraper::Node>) -> Node {
    let scraper_node = node_ref.value();
    
    // Use a helper to convert children, allowing us to skip some
    let children: Vec<Node> = node_ref.children()
        .filter_map(|child| {
            let converted = convert_ego_node(child);
            // If it's a "dummy" document node with no children, skip it
            if let NodeType::Document = converted.node_type {
                if converted.children.is_empty() {
                    return None;
                }
            }
            Some(converted)
        })
        .collect();
    
    match scraper_node {
        scraper::Node::Document => {
            Node::new(NodeType::Document, children)
        }
        scraper::Node::Element(element) => {
            let mut attributes = HashMap::new();
            for (name, value) in element.attrs() {
                attributes.insert(name.to_string(), value.to_string());
            }
            Node::element(element.name().to_string(), attributes, children)
        }
        scraper::Node::Text(text) => {
            Node::text(text.to_string())
        }
        scraper::Node::Comment(comment) => {
            Node::text(format!("<!-- {} -->", comment.comment))
        }
        _ => {
            // For Doctype and others, we return a Document node that might be filtered out by the parent
            Node::new(NodeType::Document, children)
        }
    }
}

use reqwest;
use std::env;

pub mod dom;
pub mod parser;
pub mod css;
pub mod style;
pub mod layout;

use dom::NodeType;
use css::{StyleSheet, Rule, Selector, SimpleSelector, Declaration, Value};
use style::style_tree;
use layout::{build_layout_tree, Dimensions, LayoutBox, BoxType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    println!("Fetching URL: {}", url);

    let response = reqwest::get(url).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;

    println!("Parsing HTML...");
    let dom_root = parser::parse(&body);

    println!("Computing Styles...");
    let stylesheet = get_default_stylesheet();
    let styled_root = style_tree(&dom_root, &stylesheet);

    println!("Computing Layout...");
    let mut viewport: Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 0.0; // Viewport height for layout purposes starts at 0

    let mut layout_root = build_layout_tree(&styled_root);
    layout_root.layout(viewport);

    println!("--- Layout Tree View ---");
    print_layout_tree(&layout_root, 0);
    println!("--- End Layout Tree View ---");

    Ok(())
}

fn get_default_stylesheet() -> StyleSheet {
    StyleSheet {
        rules: vec![
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("div".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("block".to_string()),
                    },
                ],
            },
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("p".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("block".to_string()),
                    },
                ],
            },
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("h1".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("block".to_string()),
                    },
                ],
            },
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("html".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("block".to_string()),
                    },
                ],
            },
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("body".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("block".to_string()),
                    },
                ],
            },
            Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: Some("head".to_string()),
                    ..Default::default()
                })],
                declarations: vec![
                    Declaration {
                        name: "display".to_string(),
                        value: Value::Keyword("none".to_string()),
                    },
                ],
            },
        ],
    }
}

fn print_layout_tree(layout_box: &LayoutBox, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let d = layout_box.dimensions.content;
    print!("{}Box: (x: {:.1}, y: {:.1}, w: {:.1}, h: {:.1})", indent_str, d.x, d.y, d.width, d.height);
    match &layout_box.box_type {
        BoxType::BlockNode(node) => {
            if let NodeType::Element(data) = &node.node.node_type {
                print!(" <{}>", data.tag_name);
            }
        }
        BoxType::InlineNode(node) => {
            if let NodeType::Element(data) = &node.node.node_type {
                print!(" <{}> (inline)", data.tag_name);
            } else if let NodeType::Text(text) = &node.node.node_type {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    print!(" \"{}\"", trimmed);
                } else {
                    print!(" (whitespace text)");
                }
            }
        }
        BoxType::AnonymousBlock => print!(" (anonymous block)"),
    }
    println!();

    for child in &layout_box.children {
        print_layout_tree(child, indent + 1);
    }
}

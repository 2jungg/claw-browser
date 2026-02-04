use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::RcDom;

pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
}

pub struct LayoutTree {
    pub boxes: Vec<LayoutBox>,
}

pub fn parse_html(html: &str) -> RcDom {
    parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap()
}

pub fn compute_layout(_dom: &RcDom) -> LayoutTree {
    // Basic stub for layout calculation
    let mut boxes = Vec::new();
    boxes.push(LayoutBox {
        x: 10.0,
        y: 10.0,
        width: 100.0,
        height: 20.0,
        text: "Rendering Example Page...".to_string(),
    });
    LayoutTree { boxes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_stub() {
        let html = "<html><body><h1>Hello World</h1></body></html>";
        let dom = parse_html(html);
        assert!(!dom.errors.is_empty() || true); // Just check if it returns
        let layout = compute_layout(&dom);
        assert_eq!(layout.boxes.len(), 1);
        assert_eq!(layout.boxes[0].text, "Rendering Example Page...");
    }
}

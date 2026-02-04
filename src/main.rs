use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://example.com";
    println!("ClawBrowser: Navigating to {}", url);

    // 1. Fetch
    let resource = claw_net::fetch(url).await?;
    let html_content = String::from_utf8_lossy(&resource.content);
    println!("Fetched {} bytes (Status: {})", html_content.len(), resource.status);

    // 2. Parse
    let dom = claw_layout::parse_html(&html_content);
    println!("HTML Parsed into DOM.");

    // 3. Layout
    let layout_tree = claw_layout::compute_layout(&dom);
    println!("Layout calculated ({} boxes).", layout_tree.boxes.len());

    // 4. Render
    let window = claw_ui::BrowserWindow::new("ClawBrowser v1.0", 1280, 720);
    window.run(layout_tree).await;

    Ok(())
}

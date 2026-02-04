use url::Url;
use anyhow::Result;

pub struct Resource {
    pub content: Vec<u8>,
    pub mime_type: String,
    pub status: u16,
}

pub async fn fetch(url: &str) -> Result<Resource> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let status = res.status().as_u16();
    let mime_type = res.headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("text/plain")
        .to_string();
    let content = res.bytes().await?.to_vec();
    
    Ok(Resource {
        content,
        mime_type,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_status() {
        // We test with a reliable domain
        let res = fetch("https://www.google.com").await;
        assert!(res.is_ok());
        let resource = res.unwrap();
        assert_eq!(resource.status, 200);
    }
}

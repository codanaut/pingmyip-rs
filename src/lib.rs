use reqwest::Error;

/// Fetches the IP details as JSON from pingmyip.io
pub async fn fetch_ip_json() -> Result<String, Error> {
    let url = "https://pingmyip.io/json";
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

/// Fetches the IP details as plain text from pingmyip.io
pub async fn fetch_ip_text() -> Result<String, Error> {
    let url = "https://pingmyip.io/txt";
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_ip_json() {
        let result = fetch_ip_json().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_fetch_ip_txt() {
        let result = fetch_ip_text().await;
        assert!(result.is_ok());
    }
}

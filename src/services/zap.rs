use crate::models::zap_models::MessagesResponse;
use reqwest::Url;
pub struct ZapService
{
    api_key: String,
    url:     Url,
    client:  reqwest::blocking::Client,
}

impl ZapService
{
    pub fn new(url: String, api_key: String) -> Self
    {
        let client = reqwest::blocking::Client::new();
        Self {
            api_key,
            url: Url::parse(&url).expect("zap url should be valid"),
            client,
        }
    }
}

impl ZapService
{
    fn get(
        &self,
        endpoint: &str,
        query: Option<&[(String, String)]>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>
    {
        info!("GET {}", endpoint);
        debug!("query => {:?}", query);
        let url = self.url.join(endpoint).unwrap();
        self.client
            .get(url)
            .query(query.unwrap_or(&[]))
            .header("X-ZAP-API-Key", &self.api_key)
            .header("Accept", "application/json")
            .send()
    }

    pub fn view_messages(
        &self,
        base_url: Option<String>,
        start: Option<u64>,
        count: Option<u64>,
    ) -> Result<MessagesResponse, reqwest::Error>
    {
        let mut query = vec![];
        add_to_query_if_exists(&mut query, "baseurl".into(), base_url);
        add_to_query_if_exists(&mut query, "start".into(), start.map(|x| x.to_string()));
        add_to_query_if_exists(&mut query, "count".into(), count.map(|x| x.to_string()));

        self.get("/JSON/core/view/messages", Some(&query))?.json()
    }
}

// Helpers

fn add_to_query_if_exists(query: &mut Vec<(String, String)>, key: String, value: Option<String>)
{
    if let Some(value) = value
    {
        query.push((key, value.into()));
    }
}

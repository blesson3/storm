use crate::services::zap::ZapService;
use regex::Regex;

pub fn run(base_url: &str)
{
    let url = "http://localhost:8080".to_string();
    let api_key = dotenv!("ZAP_API_KEY").to_string();

    let service = ZapService::new(url, api_key);
    let messages = service
        .view_messages(Some(base_url.into()), None, None)
        .unwrap();

    let uri_regex = Regex::new(r"^(\w+)\s([^ ?]+)").unwrap();

    let mut unique_endpoints = vec![];
    for message in messages.messages
    {
        let captures = uri_regex.captures(&message.request_header).unwrap();
        let method = captures[1].to_string();
        let endpoint = captures[2].to_string();

        let endpoint_full = format!("{} {}", method, endpoint);

        if !unique_endpoints.contains(&endpoint_full)
        {
            unique_endpoints.push(endpoint_full);
        }
    }

    println!("{}", unique_endpoints.join("\n"));
}

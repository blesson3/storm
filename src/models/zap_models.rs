// auto generated using app.quicktype.io

// GET zap/JSON/core/view/messages
#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesResponse
{
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message
{
    #[serde(rename = "cookieParams")]
    pub cookie_params: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "requestBody")]
    pub request_body: String,

    #[serde(rename = "requestHeader")]
    pub request_header: String,

    #[serde(rename = "responseBody")]
    pub response_body: String,

    #[serde(rename = "responseHeader")]
    pub response_header: String,

    #[serde(rename = "rtt")]
    pub rtt: String,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "timestamp")]
    pub timestamp: String,

    #[serde(rename = "type")]
    pub message_type: String,
}

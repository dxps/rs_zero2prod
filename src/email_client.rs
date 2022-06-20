use reqwest::Client;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    api_base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(api_base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            api_base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}

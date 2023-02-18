use crate::auth;

pub struct Client {
    pub client: reqwest::Client,
    pub auth_token: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
            auth_token: String::new(),
        }
    }

    pub async fn auth(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.auth_token = auth::get_auth_token(self).await?;
        Ok(())
    }
}

use crate::auth;

#[derive(Default)]
pub struct Client {
    pub client: reqwest::Client,
    pub auth_token: String,
}

impl Client {
    pub async fn auth(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.auth_token = auth::get_auth_token(self).await?;
        Ok(())
    }
}

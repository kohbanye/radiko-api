use crate::area;
use crate::auth;

#[derive(Default, Clone, Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub auth_token: String,
    pub area_id: String,
}

pub const V2_URL: &str = "https://radiko.jp/v2/";
pub const V3_URL: &str = "https://radiko.jp/v3/";

impl Client {
    pub async fn new() -> Result<Client, Box<dyn std::error::Error>> {
        let mut client: Client = Default::default();
        client.auth().await?;
        client.set_area_id().await?;

        Ok(client)
    }

    pub fn request(&self, method: reqwest::Method, url_path: &str) -> reqwest::RequestBuilder {
        if self.auth_token.is_empty() {
            panic!("auth_token is empty");
        }

        self.client
            .request(method, url_path)
            .header("X-Radiko-AuthToken", &self.auth_token)
    }

    async fn auth(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.auth_token = auth::get_auth_token(self).await?;
        Ok(())
    }

    async fn set_area_id(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.area_id = area::get_area_id().await?;
        Ok(())
    }
}

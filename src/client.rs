use crate::auth;
use crate::area;

#[derive(Default)]
pub struct Client {
    pub client: reqwest::Client,
    pub auth_token: String,
    pub area_id: String,
}

pub const V2_API_URL: &str = "https://radiko.jp/v2/api/";
pub const V3_API_URL: &str = "https://radiko.jp/v3/api/";

impl Client {
    pub fn request(&self, method: reqwest::Method, url_path: &str) -> reqwest::RequestBuilder {
        if self.auth_token.is_empty() {
            panic!("auth_token is empty");
        }

        self.client
            .request(method, url_path)
            .header("X-Radiko-AuthToken", &self.auth_token)
    }

    pub async fn auth(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.auth_token = auth::get_auth_token(self).await?;
        Ok(())
    }

    pub async fn set_area_id(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.area_id = area::get_area_id().await?;
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

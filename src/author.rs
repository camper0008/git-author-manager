#[derive(serde::Deserialize, serde::Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("name  -> '{}'\nemail -> '{}'", self.name, self.email);
        write!(f, "{s}")
    }
}

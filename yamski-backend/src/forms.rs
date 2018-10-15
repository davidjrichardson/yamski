#[derive(FromForm, Deserialize)]
pub struct AliasForm {
    pub alias: String
}
#[derive(FromForm, Deserialize)]
pub struct AliasForm {
    pub alias: String,
}

#[derive(FromForm, Deserialize)]
pub struct PlaylistItemForm {
    pub source_url: String,
}

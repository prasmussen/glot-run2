

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub version: String,
    pub image: String,
}


#[derive(Debug, serde::Deserialize)]
pub struct LanguageData {
    pub name: String,
    pub version: String,
    pub image: String,
}

pub fn new(data: &LanguageData) -> Language {
    let id = sha1_hash(&format!("{}{}", data.name, data.version));

    Language{
        id,
        name: data.name.clone(),
        version: data.version.clone(),
        image: data.image.clone(),
    }
}


fn sha1_hash(s: &str) -> String {
    sha1::Sha1::from(s).hexdigest()
}

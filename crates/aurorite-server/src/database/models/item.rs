

#[derive(Debug, Clone, toasty::Model)]
pub struct Item {
    #[key]
    pub id: uuid::Uuid,
    pub title: String, 
    #[unique]
    pub l18n_key: Option<String>,
}
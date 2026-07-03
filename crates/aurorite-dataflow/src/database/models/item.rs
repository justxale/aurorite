#[derive(Debug, Clone, toasty::Model)]
pub struct Item {
    #[key]
    pub id: uuid::Uuid,
    pub title: String,
    #[unique]
    pub i18n: Option<String>,
}

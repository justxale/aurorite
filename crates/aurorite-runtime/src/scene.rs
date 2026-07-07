use uuid::Uuid;

#[derive(Debug)]
pub struct Scene {
    pub asset: Option<String>,
    pub characters: Vec<(Uuid, bool)>
}
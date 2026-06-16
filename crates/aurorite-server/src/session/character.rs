pub use uuid::Uuid;
use aurorite_dataflow::dto::{AbilitiesDto, CharacterDto, SkillsDto};

#[derive(Clone, Debug)]
pub struct Character {
    pub id: Uuid,
    pub controlled_by: Uuid,

    pub name: String,
    pub is_enemy: bool,
    pub is_npc: bool,

    pub max_hits: u16,
    pub current_hits: u16,

    pub abilities: AbilitiesDto,
    pub skills: SkillsDto,
}

impl Character {
    pub fn from_dto(dto: CharacterDto, controlled_by: Uuid, is_enemy: bool, is_npc: bool, current_hits: u16) -> Self {
        Self {
            id: dto.id.uuid(),
            controlled_by,

            name: dto.name.unwrap_or(dto.full_name),
            is_enemy, is_npc,
            
            max_hits: dto.max_hits,
            current_hits,
            
            skills: dto.skills,
            abilities: dto.abilities,
        }
    }
}

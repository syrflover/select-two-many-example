use sea_orm::prelude::*;

use crate::entity;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "food_truck")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "entity::food::Entity")]
    Food,
}

impl Related<entity::food::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Food.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::prelude::*;

use crate::entity;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "food")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,

    pub food_truck_id: i32,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "entity::food_truck::Entity",
        from = "Column::FoodTruckId",
        to = "entity::food_truck::Column::Id",
        on_delete = "Cascade"
    )]
    FoodTruck,
}

impl Related<entity::food_truck::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FoodTruck.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

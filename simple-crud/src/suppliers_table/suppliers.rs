//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "suppliers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub supplier_id: i32,
    #[sea_orm(unique)]
    pub supplier_name: String,
    pub fruit_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::Fruits",
        from = "Column::FruitId",
        to = "crate::FruitsColumn::FruitId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Fruits,
}

impl Related<crate::Fruits> for Entity {
    fn to() -> RelationDef {
        Relation::Fruits.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
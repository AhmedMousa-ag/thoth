use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "steps")]
pub struct Model {
    #[sea_orm(primary_key, unique, auto_increment = false)]
    op_id: String,
    step_id: String,
    file_path: String,
    #[sea_orm(defualt = false)]
    is_finished: bool,
    #[sea_orm(nullable)]
    result: String, //Not sure.
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Operation,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Operation => Entity::belongs_to(super::operations::Entity)
                .from(Column::OpId)
                .to(super::operations::Column::OpId)
                .into(),
        }
    }
}

impl Related<super::operations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

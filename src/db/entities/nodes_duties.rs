use sea_orm::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "nodes_duties")]
pub struct Model {
    #[sea_orm(primary_key, unique, auto_increment = false)]
    node_id: String, // Instead of using UUID, we are already converting everything into string and UUID several times.
    op_id: String,
    #[sea_orm(default_value = false)]
    is_finished: bool,
    //TODO you might put the result here.
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

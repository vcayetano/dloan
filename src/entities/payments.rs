//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub payment_id: i32,
    pub transaction_id: i32,
    pub payment_date: DateTime,
    #[sea_orm(column_type = "Float")]
    pub payment_amount: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::loan_transactions::Entity",
        from = "Column::TransactionId",
        to = "super::loan_transactions::Column::TransactionId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LoanTransactions,
}

impl Related<super::loan_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoanTransactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

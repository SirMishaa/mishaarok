use loco_rs::schema::table_auto_tz;
use sea_orm_migration::sea_orm::EnumIter;
use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TunnelsState::Enum)
                    .values([TunnelsState::Active, TunnelsState::Inactive])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                table_auto_tz(Tunnels::Table)
                    .col(pk_auto(Tunnels::Id))
                    .col(string(Tunnels::Url))
                    .col(integer(Tunnels::UserId))
                    .col(
                        ColumnDef::new(Tunnels::State)
                            .custom(TunnelsState::Enum)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tunnels-users")
                            .from(Tunnels::Table, Tunnels::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tunnels::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TunnelsState::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tunnels {
    #[sea_orm(iden = "tunnels")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "url")]
    Url,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "state")]
    State,
}

#[derive(DeriveIden)]
pub enum TunnelsState {
    #[sea_orm(iden = "tunnels_state")]
    Enum,
    #[sea_orm(iden = "active")]
    Active,
    #[sea_orm(iden = "inactive")]
    Inactive,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

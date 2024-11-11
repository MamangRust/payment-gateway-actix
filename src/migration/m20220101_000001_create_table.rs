use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Users Table
        let users_table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::UserId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Users::Firstname).string_len(100).not_null())
            .col(ColumnDef::new(Users::Lastname).string_len(100).not_null())
            .col(
                ColumnDef::new(Users::Email)
                    .string_len(100)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Users::Password).string_len(100).not_null())
            .col(
                ColumnDef::new(Users::NocTransfer)
                    .string_len(255)
                    .unique_key()
                    .not_null()
                    .default("0"),
            )
            .col(
                ColumnDef::new(Users::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Users::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();
        manager.create_table(users_table).await?;

        // Create Topups Table
        let topups_table = Table::create()
            .table(Topups::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Topups::TopupId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Topups::UserId)
                    .integer()
                    .not_null()
                    
            )
            .col(ColumnDef::new(Topups::TopupNo).text().not_null())
            .col(ColumnDef::new(Topups::TopupAmount).integer().not_null())
            .col(ColumnDef::new(Topups::TopupMethod).text().not_null())
            .col(ColumnDef::new(Topups::TopupTime).timestamp().not_null())
            .col(
                ColumnDef::new(Topups::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-topups-user_id")
                    .from(Topups::Table, Topups::UserId)
                    .to(Users::Table, Users::UserId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .col(
                ColumnDef::new(Topups::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();
        manager.create_table(topups_table).await?;

        // Create Saldo Table
        let saldo_table = Table::create()
            .table(Saldo::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Saldo::SaldoId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Saldo::UserId)
                    .integer()
                    .not_null()
                    
            )
            .col(ColumnDef::new(Saldo::TotalBalance).integer().not_null())
            .col(ColumnDef::new(Saldo::WithdrawAmount).integer().default(0))
            .col(
                ColumnDef::new(Saldo::WithdrawTime)
                    .timestamp()
            )
            .col(
                ColumnDef::new(Saldo::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Saldo::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-saldo-user_id")
                    .from(Saldo::Table, Saldo::UserId)
                    .to(Users::Table, Users::UserId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(saldo_table).await?;

        // Create Transfers Table
        let transfers_table = Table::create()
            .table(Transfers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Transfers::TransferId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Transfers::TransferFrom)
                    .integer()
                    .not_null()
                    
            )
            .col(
                ColumnDef::new(Transfers::TransferTo)
                    .integer()
                    .not_null()
            )
            .col(
                ColumnDef::new(Transfers::TransferAmount)
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new(Transfers::TransferTime)
                    .timestamp()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Transfers::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Transfers::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-transfers-user_to")
                    .from(Transfers::Table, Transfers::TransferTo)
                    .to(Users::Table, Users::UserId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-transfers-user_from")
                    .from(Transfers::Table, Transfers::TransferFrom)
                    .to(Users::Table, Users::UserId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(transfers_table).await?;

        // Create Withdraws Table
        let withdraws_table = Table::create()
            .table(Withdraws::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Withdraws::WithdrawId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Withdraws::UserId).integer().not_null())
            .col(
                ColumnDef::new(Withdraws::WithdrawAmount)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Withdraws::WithdrawTime)
                    .timestamp()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Withdraws::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Withdraws::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-withdraws-user_id")
                    .from(Withdraws::Table, Withdraws::UserId)
                    .to(Users::Table, Users::UserId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(withdraws_table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Withdraws::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Transfers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Saldo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Topups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    UserId,
    Firstname,
    Lastname,
    Email,
    Password,
    NocTransfer,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Topups {
    Table,
    TopupId,
    UserId,
    TopupNo,
    TopupAmount,
    TopupMethod,
    TopupTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Saldo {
    Table,
    SaldoId,
    UserId,
    TotalBalance,
    WithdrawAmount,
    WithdrawTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Transfers {
    Table,
    TransferId,
    TransferFrom,
    TransferTo,
    TransferAmount,
    TransferTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Withdraws {
    Table,
    WithdrawId,
    UserId,
    WithdrawAmount,
    WithdrawTime,
    CreatedAt,
    UpdatedAt,
}


impl Default for Migration {
    fn default() -> Self {
        Migration
    }
}
use entity::test_data;
use sea_orm_migration::{async_trait, sea_orm::{DbBackend, DeriveMigrationName, Schema}, sea_query::Table, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

const DB_TYPE: DbBackend = DbBackend::Sqlite;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let schema = Schema::new(DB_TYPE);

		let table =
			schema
				.create_table_from_entity(test_data::Entity)
				.if_not_exists()
				.to_owned();

		manager
			.create_table(table)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let table =
			Table::drop()
				.table(test_data::Entity)
				.to_owned();

		manager
			.drop_table(table)
			.await
	}
}

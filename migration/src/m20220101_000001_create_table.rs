use sea_orm_migration::async_trait;
use sea_orm_migration::prelude::*;

// このマイグレーション後のエンティティ作成は
// sea-orm-cli generate entity -o ./ゴミフォルダ -t 作ったテーブル名(カンマ列挙)
// のようにするべき、
// まず、こいつらは自動で型を推測して作成してくれない(DBにあるべきままの状態で作成してくる)し
// パッケージとして作成すらしてくれない

// DeriveIdenを使用されたenumは列挙名に対応した名前(string)を定義できる
// ただし、Tableと定義するとenumの名前が適用される
#[derive(DeriveIden)]
enum TestData {
	Table,

	// Column
	Id, // i32 pk
	Username, // String
	Memo, // Text nullable
	CreatedAt, // DateTimeWithDateZone
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let table =
			Table::create()
				.table(TestData::Table)
				.if_not_exists()
				.col(
					ColumnDef::new(TestData::Id)
						.integer()
						.primary_key()
						.not_null()
				)
				.col(
					ColumnDef::new(TestData::Username)
						.string()
						.not_null()
				)
				.col(
					ColumnDef::new(TestData::Memo)
						.text()
						.null()
				)
				.col(
					ColumnDef::new(TestData::CreatedAt)
						.timestamp_with_time_zone()
						.not_null()
				)
				.to_owned(); // 必須

		manager
			.create_table(table)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let table =
			Table::drop()
				.table(TestData::Table)
				.to_owned(); // 必須

		manager
			.drop_table(table)
			.await
	}
}

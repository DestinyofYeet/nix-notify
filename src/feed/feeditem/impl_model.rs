use std::sync::LazyLock;

use django_rs::models::{
    MigrationKind, ModelMigration,
    column::{ColumnType, CreateColumn, CreateOptions},
    traits::model::Model,
};

use crate::feed::feeditem::FeedItem;

impl Model for FeedItem {
    const TABLE_NAME: &'static str = "FeedItem";

    fn get_migration() -> &'static Vec<django_rs::models::ModelMigration> {
        static MIGRATION: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "feed_name",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "message",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "package",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "commithash",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable().set_unique(),
                    ),
                    CreateColumn::new(
                        "updated",
                        ColumnType::Date,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "author",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "link",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                ]),
            )]
        });

        &MIGRATION
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}

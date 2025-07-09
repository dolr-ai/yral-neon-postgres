use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the notification table first
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(string(Notification::UserPrincipal).not_null())
                    .col(integer(Notification::Id).primary_key().auto_increment())
                    .col(boolean(Notification::Read).not_null().default(false))
                    .col(timestamp(Notification::Timestamp).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Table for the `Liked` payload variant
        manager
            .create_table(
                Table::create()
                    .table(NotificationLikedPayload::Table)
                    .if_not_exists()
                    .col(integer(NotificationLikedPayload::NotificationId).primary_key())
                    .col(string(NotificationLikedPayload::ByUserPrincipal).not_null())
                    .col(big_integer(NotificationLikedPayload::PostId).not_null())
                    // Foreign key to notification
                    .foreign_key(
                        ForeignKey::create()
                            .from(NotificationLikedPayload::Table, NotificationLikedPayload::NotificationId)
                            .to(Notification::Table, Notification::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Table for the `VideoUploadPayload` variant
        manager
            .create_table(
                Table::create()
                    .table(NotificationVideoUploadPayload::Table)
                    .if_not_exists()
                    .col(integer(NotificationVideoUploadPayload::NotificationId).primary_key())
                    .col(big_integer(NotificationVideoUploadPayload::VideoId).not_null())
                    // Foreign key to notification
                    .foreign_key(
                        ForeignKey::create()
                            .from(NotificationVideoUploadPayload::Table, NotificationVideoUploadPayload::NotificationId)
                            .to(Notification::Table, Notification::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_notification_user_principal")
                    .table(Notification::Table)
                    .col(Notification::UserPrincipal)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop child tables first (respect FK constraints) and ignore if they don't exist
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(NotificationLikedPayload::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(NotificationVideoUploadPayload::Table)
                    .to_owned(),
            )
            .await?;

        // Drop index (if it exists) before dropping the main table
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_notification_user_principal")
                    .table(Notification::Table)
                    .to_owned(),
            )
            .await?;

        // Finally drop the main table (if it exists)
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Notification::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Notification {
    UserPrincipal,
    Id,
    Table,
    Read,
    Timestamp,
}

#[derive(Iden)]
pub enum NotificationLikedPayload {
    #[iden = "notification_liked_payload"]
    Table,
    NotificationId,
    ByUserPrincipal,
    PostId,
}

#[derive(Iden)]
pub enum NotificationVideoUploadPayload {
    #[iden = "notification_video_upload_payload"]
    Table,
    NotificationId,
    VideoId,
}


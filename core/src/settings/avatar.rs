use crate::AppCore;
use base64::Engine;
use database::entity::users;
use error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::Expr;
use session::Session;

impl AppCore {
    pub async fn setting_avatar_upload(
        &self,
        session: Session,
        file: Vec<u8>,
    ) -> Result<(), AppError> {
        let user = self.user_context(session).await?;
        users::Entity::update_many()
            .col_expr(
                users::Column::AvatarUrl,
                Expr::value(base64::engine::general_purpose::STANDARD.encode(file)),
            )
            .filter(users::Column::Uid.eq(user.user_uid))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

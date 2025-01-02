use actix_session::Session;
use sea_orm::prelude::async_trait::async_trait;

type Context = Session;
#[async_trait]
pub trait UsersTrait {
    async fn login(ctx: Context);
    async fn register(ctx: Context);
    async fn logout(ctx: Context);
    async fn options(ctx: Context);
}

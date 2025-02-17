use poem::{get, handler, post, Response, Route};
use poem::http::StatusCode;

pub(crate) const GIT_ROOT: &str = "./data";

pub enum GitPack {
    UploadPack,
    ReceivePack
}

pub fn git_router() -> Route {
    Route::new()
        .at("/:owner/:repo/git-upload-pack", post(pack::pack))
        .at("/:owner/:repo/git-receive-pack", post(pack::pack))
        .at("/:owner/:repo/info/refs", get(refs::refs))
        .at("/:owner/:repo/HEAD", get(todo))
        .at("/:owner/:repo/objects/info/alternates", get(todo))
        .at("/:owner/:repo/objects/info/http-alternates", get(todo))
        .at("/:owner/:repo/objects/info/packs", get(todo))
        .at("/:owner/:repo/objects/info/{file:[^/]*}", get(todo))
        .at("/:owner/:repo/objects/{head:[0-9a-f]{2}}/{hash:[0-9a-f]{38}}", get(todo))
        .at("/:owner/:repo/objects/pack/pack-{file:[0-9a-f]{40}}.pack", get(todo))
        .at("/:owner/:repo/objects/pack/pack-{file:[0-9a-f]{40}}.idx", get(todo))
}

#[handler]
async fn todo() -> Response {
    Response::builder()
        .status(StatusCode::BAD_GATEWAY)
        .body("Seems like an asteroid destroyed the ancient git protocol")
}

pub mod pack;
pub mod refs;



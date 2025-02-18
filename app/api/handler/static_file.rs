use crate::app::api::write::AppWrite;
use crate::app::http::GIT_ROOT;
use crate::app::services::AppState;
use crate::model::users::users;
use poem::http::{HeaderMap, StatusCode};
use poem::session::Session;
use poem::web::Multipart;
use poem::{handler, web, IntoResponse};
use std::io::Write;

#[handler]
pub async fn upload_avatar(
    mut payload: Multipart,
    state: web::Data<&AppState>,
    session: &Session,
)  -> impl IntoResponse {

    let uid = match session.get::<String>("user"){
        Some(uid) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::<()>::unauthorized("请先登录".to_string())
            }
        },
        None => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };
    let avatar = format!("{}/static",GIT_ROOT);
    if !std::path::Path::new(&avatar).exists() {
        std::fs::create_dir_all(&avatar).expect("创建目录失败");
    }
    let mut avatar = format!("{}/{}",avatar,uid);
    while let Ok(Some(field)) = payload.next_field().await {
        let file = field.file_name().unwrap_or(&format!("{}.png",uid)).to_string();
        avatar = format!("{}-{}",avatar,file);
        let data = field.bytes().await.unwrap_or(vec![]);
        let mut fs = std::fs::File::options()
            .append(true)
            .create(true)
            .open(&avatar).unwrap();
        fs.write_all(&data).unwrap();
    }
    match state.user_avatar_update(uid,format!("/api/static/img/{}",avatar.split("/").map(|x|x.to_string()).filter(|x|!x.is_empty()).collect::<Vec<_>>().last().unwrap_or(&"".to_string()))).await {
        Ok(_) => AppWrite::ok_msg("上传成功".to_string()),
        Err(_) => AppWrite::error("上传失败".to_string()),
    }
}


#[handler]
pub async fn down_avatar(
    path: web::Path<String>,
) -> impl IntoResponse {
    let avatar = format!("{}/static/{}",GIT_ROOT,path.0);
    dbg!(&avatar);
    let mut header = HeaderMap::new();
    header.append("Content-Type", "image/png".parse().unwrap());
    if std::path::Path::new(&avatar).exists() {
        let bytes = std::fs::read(&avatar).unwrap();
        (StatusCode::OK, header, bytes)
        
    }else{
        (StatusCode::NOT_FOUND,header, "404 Not Found".into())
    }
}

use actix_web::web;
use crate::http::backend::head::get_text_file;
use crate::http::backend::object_pack::objects_pack;
use crate::http::backend::object_pack_info::objects_info_packs;

pub mod git_upload_pack;
pub mod git_receive_pack;
pub mod info_refs;
pub mod head;
pub mod object_pack_info;
pub mod object_pack;
pub fn routes(
    cfg: &mut web::ServiceConfig
)
{
    cfg
        .route("/git-upload-pack", web::to(git_upload_pack::git_upload_pack))
        .route("/git-receive-pack", web::to(git_receive_pack::git_receive_pack))
        .route("/info/refs", web::to(info_refs::info_refs))
        .route("/HEAD", web::to(head::get_text_file))
        .route("objects/info/alternates", web::to(get_text_file))
        .route("objects/info/http-alternates", web::to(get_text_file))
        .route("objects/info/packs", web::to(objects_info_packs))
        .route("objects/info/{rest:.*}", web::to(get_text_file))
        .route("objects/pack/{pack}", web::to(objects_pack))
    ;
}
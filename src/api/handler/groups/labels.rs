use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto::GroupsLabels;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups/{group}/labels",
    params(
        ("group" = String, Path, description = "Group Name"),
    ),
    responses(
        (status = 200, description = "Success", body = [GroupsLabels]),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_labels(
    service: web::Data<MetaService>,
    group: web::Path<String>
) -> impl Responder
{
    let group_id = match service.group_service().name_to_uid(group.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    let labels = service.group_service().labels(group_id).await;
    match labels{
        Ok(labels) => AppWrite::ok(labels),
        Err(e) => AppWrite::fail(e.to_string())
    }
}
#[utoipa::path(
    post,
    tag = "groups",
    path = "/api/v1/groups/{group}/labels",
    params(
        ("group" = String, Path, description = "Group Name"),
    ),
    request_body = GroupsLabels,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_labels_create(
    service: web::Data<MetaService>,
    group: web::Path<String>,
    label: web::Json<GroupsLabels>
) -> impl Responder
{
    let group_id = match service.group_service().name_to_uid(group.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    let (label, color) = (label.labels.clone(), label.color.clone());
    match service.group_service().label_create(group_id, label,color).await{
        Ok(_) => AppWrite::ok("success".to_string()),
        Err(e) => AppWrite::fail(e.to_string())
    }
}

#[utoipa::path(
    delete,
    tag = "groups",
    path = "/api/v1/groups/{group}/labels/{label}",
    params(
        ("group" = String, Path, description = "Group Name"),
        ("label" = Uuid, Path, description = "Label UID"),
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_labels_delete(
    service: web::Data<MetaService>,
    path: web::Path<(String, Uuid)>,
) -> impl Responder
{
    let (group, label) = path.into_inner();
    let _ = match service.group_service().name_to_uid(group).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    match service.group_service().label_delete(label).await{
        Ok(_) => AppWrite::ok("success".to_string()),
        Err(e) => AppWrite::fail(e.to_string())
    }
}
#[utoipa::path(
    put,
    tag = "groups",
    path = "/api/v1/groups/{group}/labels/{label}",
    params(
        ("group" = String, Path, description = "Group Name"),
        ("label" = Uuid, Path, description = "Label UID"),
    ),
    request_body = GroupsLabels,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_labels_update(
    service: web::Data<MetaService>,
    path: web::Path<(String, Uuid)>,
    labels: web::Json<GroupsLabels>
) -> impl Responder
{
    let (group, label) = path.into_inner();
    let _ = match service.group_service().name_to_uid(group).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    match service.group_service().label_update(label, labels.labels.clone(), labels.color.clone()).await{
        Ok(_) => AppWrite::ok("success".to_string()),
        Err(e) => AppWrite::fail(e.to_string())
    }
}
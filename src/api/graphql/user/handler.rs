use crate::api::app_write::AppWrite;
use crate::api::dto::repo_dto::RepoBranchOv;
use crate::api::graphql::user::dto::UserGraphqlQuery;
use crate::api::graphql::user::ov::{GraphQLEmail, GraphQLUserData, GraphQLUserGroup, GraphQLUserKeys, GraphQLUserModel, GraphQLUserProfile, GraphQLUserRepo};
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;
use actix_session::Session;
use actix_web::{web, Responder};


#[utoipa::path(
    post,
    path = "/api/graphql/user",
    request_body = UserGraphqlQuery,
    tags = ["graphql"],
    responses(
        (status = 200, description = "Get repo branch commits", body = GraphQLUserModel),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("owner" = String, description = "Repo owner"),
        ("repo"= String, description= "Repo name"),
    )
)]
pub async fn graphql_user_handler(
    session: Session,
    query: web::Json<UserGraphqlQuery>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    let user_uid = {
        if let Some(username) = query.username.clone(){
            if username.is_empty(){
                match check_session(session).await{
                    Ok(session)=> session.uid,
                    Err(e)=> return AppWrite::<GraphQLUserModel>::unauthorized(e.to_string())
                }
            }else {
                match service.user_service().username_to_uid(username).await{
                    Ok(model)=> model,
                    Err(e)=> return AppWrite::fail(e.to_string())
                }
            }
        }else {
            match check_session(session).await{
                Ok(session)=> session.uid,
                Err(e)=> return AppWrite::<GraphQLUserModel>::unauthorized(e.to_string())
            }
        }
    };
    let mut result = GraphQLUserModel{
        profile: None,
        repo: None,
        keys: None,
        data: None,
        email: None,
        group: None,
    };
    if query.profile{
        let profile = match service.user_service()._user_private(user_uid).await{
            Ok(model)=> model,
            Err(e)=> return AppWrite::fail(e.to_string())
        };
        result.profile = Some(GraphQLUserProfile{
            uid: profile.uid,
            name: profile.name,
            username: profile.username,
            avatar: profile.avatar,
            phone: profile.phone,
            status: profile.status,
            website: profile.website,
            company: profile.company,
            description: profile.description,
            localtime: profile.localtime,
            timezone: profile.timezone,
            theme: profile.theme,
            pro: profile.pro,
            created_at: profile.created_at.unix_timestamp(),
            updated_at: profile.updated_at.unix_timestamp(),
            lastlogin: profile.lastlogin.unix_timestamp(),
            is_groups: profile.is_groups,
        });
    }
    if query.repo{
        let mut repo_models = Vec::new();
        let repos_id = match service.user_service().user_data(user_uid).await{
            Ok(model)=> model.repo,
            Err(e)=> return AppWrite::fail(e.to_string())
        };
        for repo_id in repos_id{
            match service.repo_service().info(repo_id).await{
                Ok(model)=> {
                    let uid = model.uid.clone();
                    let data = service.repo_service().branch(model.owner.clone(), model.name.clone()).await;
                    let branch_models = match data{
                        Ok(models)=> {
                            models
                                .iter()
                                .map(|x|{
                                    RepoBranchOv{
                                        uid: x.uid,
                                        branch: x.branch.clone(),
                                        protect: x.protect,
                                        visible: x.visible,
                                        head: x.head,
                                        created_at: x.created_at.unix_timestamp(),
                                        updated_at: x.updated_at.unix_timestamp(),
                                    }
                                })
                                .collect::<Vec<_>>()
                        },
                        Err(_)=> continue
                    };
                    let model = GraphQLUserRepo{
                        uid,
                        name: model.name,
                        description: model.description,
                        owner: model.owner,
                        branch: branch_models,
                        commit: model.commit,
                        head_hash: model.head_hash,
                        ssh_path: model.ssh_path,
                        http_path: model.http_path,
                        star: model.star,
                        fork: model.fork,
                        is_fork: model.is_fork,
                        fork_from: model.fork_from,
                        watch: model.watch,
                        issue: model.issue,
                        open_issue: model.open_issue,
                        close_issue: model.close_issue,
                        pr: model.pr,
                        open_pr: model.open_pr,
                        close_pr: model.close_pr,
                        is_empty: model.is_empty,
                        visible: model.visible,
                        topic: model.topic,
                        size: model.size,
                        created_at: model.created_at.unix_timestamp(),
                        updated_at: model.updated_at.unix_timestamp(),
                    };
                    repo_models.push(model);
                },
                Err(_)=> continue
            }
        }
        result.repo = Some(repo_models);
    }
    if query.keys{
        let keys = match service.user_service().list_key(user_uid).await{
            Ok(model)=> model,
            Err(e)=> return AppWrite::fail(e.to_string())
        }
            .iter()
            .map(|x|{
                GraphQLUserKeys{
                    uid: x.uid,
                    created_at: x.created_at.clone(),
                    head: x.head.clone(),
                    last_use: x.last_use.clone(),
                }
            })
            .collect::<Vec<_>>();
        result.keys = Some(keys);
    }
    if query.data{
        let data = match service.user_service().user_data(user_uid).await{
            Ok(model)=> model,
            Err(e)=> return AppWrite::fail(e.to_string())
        };
        result.data = Some(GraphQLUserData{
            uid: data.uid,
            repo: data.repo,
            project: data.project,
            issue: data.issue,
            pr: data.pr,
            commit: data.commit,
            tag: data.tag,
            star: data.star,
            follow: data.follow,
            following: data.following,
            watcher: data.watcher,
        });
    }
    if query.email{
        let emails = match service.user_service().email(user_uid).await{
            Ok(model)=> model,
            Err(e)=> return AppWrite::fail(e.to_string())
        }
            .iter()
            .map(|x|{
                GraphQLEmail{
                    uid: x.uid,
                    user_id: x.user_id,
                    name: x.name.clone(),
                    email: x.email.clone(),
                    is_public: x.is_public,
                    verified: x.verified,
                    bind_at: x.bind_at.clone(),
                }
            })
            .collect::<Vec<_>>();
        result.email = Some(emails);
    }
    if query.groups{
        let group = match service.group_service().find_member(user_uid).await{
            Ok(model)=> model,
            Err(e)=> return AppWrite::fail(e.to_string())
        }
            .iter()
            .map(|x|{
                GraphQLUserGroup{
                    name: x.name.clone(),
                    username: x.username.clone(),
                    theme: x.theme.clone(),
                    website: x.website.clone(),
                    company: x.company.clone(),
                    description: x.description.clone(),
                    localtime: x.localtime.clone(),
                    timezone: x.timezone.clone(),
                }
            })
            .collect::<Vec<_>>();
        result.group = Some(group);
    }
    
    AppWrite::ok(result)
}
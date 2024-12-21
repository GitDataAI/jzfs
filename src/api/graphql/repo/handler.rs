use crate::api::app_write::AppWrite;
use crate::api::graphql::repo::dto::GraphQLRepoQuery;
use crate::api::graphql::repo::ov::{GraphQLRepoBranchOv, GraphQLRepoCommits, GraphQLRepoData, GraphQLRepoLicense, GraphQLRepoModel, GraphQLRepoProfile};
use crate::metadata::service::MetaService;
use actix_web::{web, Responder};

#[utoipa::path(
    post,
    path = "/api/graphql/repo",
    request_body = GraphQLRepoQuery,
    tags = ["graphql"],
    responses(
        (status = 200, description = "Get repo branch commits", body = GraphQLRepoModel),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("owner" = String, description = "Repo owner"),
        ("repo"= String, description= "Repo name"),
    )
)]
pub async fn graphql_repo_handler(
    query: web::Json<GraphQLRepoQuery>,
    service: web::Data<MetaService>
)
-> impl Responder
{
    let (owner, repo) = (query.owner.clone(), query.repo.clone());
    let mut result = GraphQLRepoModel{
        owner: owner.clone(),
        repo: repo.clone(),
        profile: None,
        data: None,
        branchs: None,
        tree: None,
        license: None,
        readme: None,
    };
    let repo_id = match service.repo_service().owner_name_by_uid(owner.clone(), repo.clone()).await{
        Ok(uid)=> uid,
        Err(e)=> return AppWrite::<GraphQLRepoModel>::fail(e.to_string())
    };
    let profile = match service.repo_service().info(repo_id).await{
        Ok(model)=> model,
        Err(e)=> return AppWrite::fail(e.to_string())
    };
    if query.profile{
        result.profile = Some(GraphQLRepoProfile{
            uid: profile.uid,
            name: profile.name,
            description: profile.description,
            owner: profile.owner,
            head_hash: profile.head_hash,
            visible: profile.visible,
            ssh_path: profile.ssh_path,
            http_path: profile.http_path,
            created_at: profile.created_at.unix_timestamp(),
            updated_at: profile.updated_at.unix_timestamp(),
        })
    }
    if query.data{
        result.data = Some(GraphQLRepoData{
            commit: profile.commit,
            star: profile.star,
            fork: profile.fork,
            is_fork: profile.is_fork,
            fork_from: profile.fork_from,
            watch: profile.watch,
            issue: profile.issue,
            open_issue: profile.open_issue,
            close_issue: profile.close_issue,
            pr: profile.pr,
            open_pr: profile.open_pr,
            close_pr: profile.close_pr,
            is_empty: profile.is_empty,
            topic: profile.topic,
            size: profile.size,
        })
    }
    if let Some(bra) =  query.branchs.clone(){
        let branchs = match service.repo_service().branch(owner.clone(), repo.clone()).await{
            Ok(branchs)=> branchs,
            Err(e)=> return AppWrite::fail(e.to_string())
        };
        let mut res = Vec::new();
        for branch in branchs{
            let mut br = GraphQLRepoBranchOv{
                uid: branch.uid,
                branch: branch.branch.clone(),
                protect: branch.protect,
                visible: branch.visible,
                head: branch.head,
                created_at: branch.created_at.unix_timestamp(),
                updated_at: branch.updated_at.unix_timestamp(),
                commit: vec![]
            };
            if let Some(commit) = bra.commit.clone(){
                let commit = match service.repo_service().list_commits(repo_id, branch.branch.clone(), commit.offset, commit.size).await{
                    Ok(commit)=> commit,
                    Err(e)=> return AppWrite::fail(e.to_string())
                }
                    .iter()
                    .map(|x|{
                        GraphQLRepoCommits{
                            uid: x.uid,
                            bio: x.bio.clone(),
                            commit_user: x.commit_user.clone(),
                            commit_email: x.commit_email.clone(),
                            commit_id: x.commit_id.clone(),
                            created_at: x.created_at.unix_timestamp(),
                        }
                    })
                    .collect::<Vec<_>>();
                br.commit = commit;
            }
            res.push(br);
        }
        result.branchs = Some(res);
    }
    if let Some(tree) = query.tree.clone(){
        match service.repo_service().tree(repo_id, tree.branch, tree.commit).await{
            Ok(tree)=> result.tree = Some(tree),
            Err(e)=> return AppWrite::fail(e.to_string())
        }
    }
    if query.license{
        match service.repo_service().licenses(repo_id).await{
            Ok(license)=> {
                result.license = Some(license.into_iter().map(|x|{
                    GraphQLRepoLicense{
                        uid: x.uid,
                        name: x.name,
                        license: x.license,
                        created_at: x.created_at.unix_timestamp(),
                        updated_at: x.updated_at.unix_timestamp(),
                    }
                }).collect::<Vec<_>>())
            },
            Err(e)=> return AppWrite::fail(e.to_string())
        }
    }
    if let Some(branch) = query.readme.clone(){
        match service.repo_service().readme(repo_id,branch).await{
            Ok(readme)=> result.readme = Some(readme),
            Err(e)=> return AppWrite::fail(e.to_string())
        }
    }
    AppWrite::ok(result)
}
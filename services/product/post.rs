use sea_orm::ColumnTrait;
use std::io;
// use git2::Oid;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::OnceCell;
use uuid::Uuid;
use crate::model::product::data_product;
use crate::model::repository::repository;
use crate::services::AppState;


#[derive(Deserialize,Serialize)]
pub struct DataProductPostParma {
    pub name: String,
    pub description: Option<String>,
    pub license: String,
    pub price: Option<i64>,
    pub hash: String,
    pub r#type: String,
}

pub struct DataProductJobs {
    user_uid: Uuid,
    parma: DataProductPostParma,
    repo_uid: Uuid,
}


pub static DATA:OnceCell<DataProductPost> = OnceCell::const_new();
pub struct DataProductPost {
    tx: UnboundedSender<DataProductJobs>
}

impl DataProductPost {
    pub async fn init(app: AppState) {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<DataProductJobs>();
        tokio::spawn(async move {
            let app = app.clone();
            while let Some(parma) = rx.recv().await {
                let _ = app.product_data_post(parma.user_uid, parma.parma, parma.repo_uid).await;
            }
        });
        DATA.get_or_init(||async {
            Self {
                tx,
            }
        }).await;
    }
    pub async fn
    send(
        user_uid: Uuid,
        parma: DataProductPostParma,
        repo_uid: Uuid,
    ) -> io::Result<()> {
        DATA.get().unwrap().tx.send(DataProductJobs {
            user_uid,
            parma,
            repo_uid,
        }).map_err(|_| io::Error::new(io::ErrorKind::Other, "send error"))
    }
}

impl AppState {
    pub async fn product_data_post(
        &self,
        user_uid: Uuid,
        parma: DataProductPostParma,
        repo_uid: Uuid,
    ) -> io::Result<()> {
        // let repo = repository::Entity::find_by_id(repo_uid)
        //     .one(&self.read)
        //     .await
        //     .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "repository not found"))?
        //     .ok_or(io::Error::new(io::ErrorKind::NotFound, "repository not found"))?;

        // let path = format!(
        //     "{}/{}/{}",
        //     crate::http::GIT_ROOT,
        //     repo.node_uid,
        //     repo.uid
        // );
        // let hash = parma.hash.clone();
        match self.product_data_publish(
            user_uid,
            parma,
            repo_uid,
        )
            .await
        {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                // std::fs::remove_file(format!("{}/{}/{}.zip", path, "product", hash)).ok();
                // std::fs::remove_dir(format!("{}/{}/{}", path, "product", hash)).ok();
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn product_data_publish(
        &self,
        user_uid: Uuid,
        parma: DataProductPostParma,
        repo_uid: Uuid,) -> io::Result<()> {
        if data_product::Entity::find()
            .filter(
                Condition::all()
                    .add(data_product::Column::Owner.eq(user_uid))
                    .add(data_product::Column::RepositoryUid.eq(repo_uid))
                    .add(data_product::Column::Name.eq(parma.name.clone()))
                    .add(data_product::Column::Hash.eq(parma.hash.clone()))
            )
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "data product error"))?
            .is_some()
        {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "this hash as data product already exists"));
        }
        let repo = repository::Entity::find_by_id(repo_uid)
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "repository not found"))?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "repository not found"))?;
        match self.user_access_owner_model(user_uid).await {
            Ok(x) => {
                if !x.iter().any(|x| x.repos.iter().any(|x|x.uid == repo_uid)){
                    return Err(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"));
                }
            },
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"));
            }
        }
        let path = format!(
            "{}/{}/{}",
            crate::http::GIT_ROOT,
            repo.node_uid,
            repo.uid
        );
        let blob = crate::blob::GitBlob::new(path.into())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let size = blob.size(parma.hash.clone())?;
        // let oid = match Oid::from_str(&parma.hash) {
        //     Ok(oid) => oid,
        //     Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid hash")),
        // };
        // blob.post_product(oid)?;
        let product_model = data_product::ActiveModel {
            uid: Set(Uuid::new_v4()),
            name: Set(parma.name),
            description: Set(parma.description),
            license: Set(parma.license),
            price: Set(parma.price),
            hash: Set(parma.hash),
            size: Set(size),
            owner: Set(user_uid),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            repository_uid: Set(repo_uid),
            r#type: Set(parma.r#type),
        };
        let _ = product_model.insert(&self.write).await;
        Ok(())
    }
}
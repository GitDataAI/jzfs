use log::info;
use mongodb::{Collection, Database};
use crate::config::CFG;
use crate::metadata::mongo::repotree::RepoTreeModel;

pub static MONGODB: tokio::sync::OnceCell<MongoDBClient> = tokio::sync::OnceCell::const_new();



#[derive(Clone)]
pub struct MongoDBClient{
    pub repo: Database,
    pub tree: Collection<RepoTreeModel>
}

impl MongoDBClient{
    pub async fn init() -> MongoDBClient {
        info!("Start Connect Mongodb");
        let cfg = CFG.get().unwrap().clone();
        let client = mongodb::Client::with_uri_str(cfg.mongodb.format()).await.expect("Failed to connect to MongoDB");
        let repo = client.database("repo");
        let tree = repo.collection::<RepoTreeModel>("RepoTree");
        info!("Connected to MongoDB for Database RepoTree");
        let result = Self{
            repo,
            tree,
        };
        MONGODB.get_or_init(||async { 
            result.clone()
        }).await.clone()
    }
}

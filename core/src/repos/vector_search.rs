use crate::{AppCore, Paginator};
use anyhow::anyhow;
use database::entity::{git_repo, repo_features, user_interactions};
use database::{git_repo_stats, user_repo, users};
use error::AppError;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sea_orm::prelude::PgVector;
use sea_orm::sea_query::{OnConflict, SimpleExpr};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::{Condition, PaginatorTrait, Set};
use serde_json::json;
use session::Session;
use std::hash::{DefaultHasher, Hasher};
use uuid::Uuid;

impl AppCore {
    pub async fn repo_vector_search(
        &self,
        session: Session,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let user_id = match self.user_context(session).await {
            Ok(u) => Some(u.user_uid),
            Err(_) => None,
        };
        if let Some(user_id) = user_id {
            let user_has_interactions = user_interactions::Entity::find()
                .filter(user_interactions::Column::UserId.eq(user_id))
                .limit(1)
                .one(&self.db)
                .await?
                .is_some();

            if user_has_interactions {
                return self.search_by_user_similarity(user_id, paginator).await;
            }
        }
        self.search_by_updated_time(paginator).await
    }
    async fn search_by_updated_time(
        &self,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let repos = git_repo::Entity::find()
            .order_by_desc(git_repo::Column::UpdatedAt)
            .limit(paginator.page_size)
            .offset(paginator.page_size * paginator.page)
            .all(&self.db)
            .await?;
        let total = git_repo::Entity::find().count(&self.db).await?;
        let mut result = Vec::new();
        for repo in repos {
            match user_repo::Entity::find()
                .filter(Condition::all().add(user_repo::Column::RepoUid.eq(repo.uid)))
                .one(&self.db)
                .await
            {
                Ok(Some(owner)) => {
                    match users::Entity::find_by_id(owner.user_uid)
                        .one(&self.db)
                        .await
                    {
                        Ok(Some(owner)) => {
                            let state = git_repo_stats::Entity::find()
                                .filter(git_repo_stats::Column::RepoUid.eq(repo.uid))
                                .one(&self.db)
                                .await?;
                            let json = json!({
                                "repo": repo,
                                "owner": {
                                    "uid": owner.uid,
                                    "username": owner.username,
                                    "avatar_url": owner.avatar_url,
                                },
                                "state": state,
                            });
                            result.push(json);
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
        Ok(json!({
            "total": total,
            "items": result,
            "page": paginator.page,
            "page_size": paginator.page_size
        }))
    }

    async fn search_by_user_similarity(
        &self,
        user_id: Uuid,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let user_interactions = user_interactions::Entity::find()
            .filter(user_interactions::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        let repo_ids_with_weights: Vec<(Uuid, f32)> = user_interactions
            .iter()
            .map(|interaction| (interaction.repo_id, interaction.weight))
            .collect();

        let repo_uids: Vec<Uuid> = repo_ids_with_weights
            .iter()
            .map(|(repo_id, _)| *repo_id)
            .collect();

        let all_repo_features = repo_features::Entity::find().all(&self.db).await?;

        let mut repo_vector_map = std::collections::HashMap::new();
        for feature in &all_repo_features {
            repo_vector_map.insert(feature.repo_uid, feature.vector.clone());
        }

        let mut similar_repos = Vec::new();
        for i in 0..repo_uids.len() {
            if let Some(vec1) = repo_vector_map.get(&repo_uids[i]) {
                for j in i + 1..repo_uids.len() {
                    if let Some(vec2) = repo_vector_map.get(&repo_uids[j]) {
                        let similarity = self.calculate_vector_similarity(vec1, vec2);
                        let weight1 = repo_ids_with_weights[i].1;
                        let weight2 = repo_ids_with_weights[j].1;
                        let adjusted_similarity = similarity * (weight1 + weight2) / 2.0;

                        similar_repos.push((repo_uids[j], adjusted_similarity));
                        similar_repos.push((repo_uids[i], adjusted_similarity));
                    }
                }
            }
        }
        if similar_repos.is_empty() {
            return self.search_by_updated_time(paginator).await;
        }
        similar_repos.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut rng = thread_rng();
        let mut unique_repos = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for (repo_id, _) in similar_repos {
            if !seen.contains(&repo_id) {
                seen.insert(repo_id);
                unique_repos.push(repo_id);
            }
        }
        unique_repos.shuffle(&mut rng);
        let start = (paginator.page_size * paginator.page) as usize;
        let end = start + paginator.page_size as usize;
        let paginated_repo_ids: Vec<Uuid> = if start < unique_repos.len() {
            if end < unique_repos.len() {
                unique_repos[start..end].to_vec()
            } else {
                unique_repos[start..].to_vec()
            }
        } else {
            Vec::new()
        };

        let repos = git_repo::Entity::find()
            .filter(git_repo::Column::Uid.is_in(paginated_repo_ids.clone()))
            .all(&self.db)
            .await?;
        let mut repo_map = std::collections::HashMap::new();
        for repo in repos {
            repo_map.insert(repo.uid, repo);
        }
        let items = paginated_repo_ids
            .into_iter()
            .filter_map(|uid| repo_map.remove(&uid))
            .collect::<Vec<_>>();

        let total = unique_repos.len() as i64;
        let mut result = Vec::new();
        for (repo_uid, model) in repo_map {
            match user_repo::Entity::find()
                .filter(Condition::all().add(user_repo::Column::RepoUid.eq(repo_uid)))
                .one(&self.db)
                .await
            {
                Ok(Some(owner)) => {
                    match users::Entity::find_by_id(owner.user_uid)
                        .one(&self.db)
                        .await
                    {
                        Ok(Some(owner)) => {
                            let state = git_repo_stats::Entity::find()
                                .filter(git_repo_stats::Column::RepoUid.eq(repo_uid))
                                .one(&self.db)
                                .await?
                                .ok_or(git_repo_stats::Model {
                                    uid: Uuid::nil(),
                                    repo_uid: repo_uid,
                                    stars: 0,
                                    watches: 0,
                                    forks: 0,
                                    created_at: Default::default(),
                                    updated_at: Default::default(),
                                });
                            let json = json!({
                                "repo": model,
                                "owner": {
                                    "uid": owner.uid,
                                    "username": owner.username,
                                    "avatar_url": owner.avatar_url,
                                },
                                "state": state,
                            });
                            result.push(json);
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
        Ok(serde_json::json!({
            "total": total,
            "items": items,
            "page": paginator.page,
            "page_size": paginator.page_size
        }))
    }

    fn calculate_vector_similarity(&self, vec1: &PgVector, vec2: &PgVector) -> f32 {
        let vec1_data = vec1.as_slice();
        let vec2_data = vec2.as_slice();

        let min_len = std::cmp::min(vec1_data.len(), vec2_data.len());
        let mut dot_product = 0.0;

        for i in 0..min_len {
            dot_product += vec1_data[i] * vec2_data[i];
        }

        dot_product
    }

    pub async fn repo_generate_embedding(&self, repo_uid: Uuid) -> Result<(), AppError> {
        let repo = git_repo::Entity::find()
            .filter(git_repo::Column::Uid.eq(repo_uid))
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("Repo not found")))?;
        let embedding = self.generate_embedding_for_repo(&repo).await?;
        let now = chrono::Utc::now();
        let feature_model = repo_features::ActiveModel {
            repo_uid: Set(repo_uid),
            vector: Set(embedding.clone()),
            meta: Set(serde_json::json!({"generated_at": now.to_string()})),
            updated_at: Set(now.naive_utc()),
        };
        let _result = repo_features::Entity::insert(feature_model)
            .on_conflict(
                OnConflict::column(repo_features::Column::RepoUid)
                    .values(vec![
                        (
                            repo_features::Column::Vector,
                            SimpleExpr::Value(sea_orm::Value::Vector(Some(Box::new(embedding)))),
                        ),
                        (
                            repo_features::Column::Meta,
                            SimpleExpr::Value(sea_orm::Value::Json(Some(Box::new(
                                serde_json::json!({"generated_at": now.to_string()}),
                            )))),
                        ),
                        (
                            repo_features::Column::UpdatedAt,
                            SimpleExpr::Value(sea_orm::Value::ChronoDateTime(Some(Box::from(
                                now.naive_utc(),
                            )))),
                        ),
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn generate_embedding_for_repo(
        &self,
        repo: &git_repo::Model,
    ) -> Result<PgVector, AppError> {
        let repo_text = format!(
            "Repository: {} by {}\nDescription: {}",
            repo.repo_name,
            repo.namespace,
            repo.description.as_deref().unwrap_or("")
        );
        let vector = self.text_to_vector(&repo_text).await?;
        let pg_vector = PgVector::from(vector);
        Ok(pg_vector)
    }

    async fn text_to_vector(&self, text: &str) -> Result<Vec<f32>, AppError> {
        const VECTOR_DIM: usize = 128;
        let mut vector = vec![0.0; VECTOR_DIM];
        for (_i, c) in text.chars().enumerate() {
            let mut hasher = DefaultHasher::new();
            hasher.write(c.to_string().as_bytes());
            let hash = hasher.finish();
            let index = (hash % VECTOR_DIM as u64) as usize;
            let value = (hash as f32 / u64::MAX as f32) * 2.0 - 1.0;
            vector[index] += value;
        }
        let norm: f32 = vector.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut vector {
                *val /= norm;
            }
        }
        Ok(vector)
    }
}

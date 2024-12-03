use sea_orm::*;
use crate::api::dto::common::ListOption;
use crate::api::dto::search::SearchTeamOptions;
use crate::api::service::teams::TeamService;
use crate::metadata::model::teams::teams;
use crate::metadata::model::teams::teams::Model;

impl TeamService {
    pub async fn list(&self, option: ListOption<SearchTeamOptions>) -> Vec<Model> {
        let filter = option.filter;
        let mut result = vec![];
        let mut models = teams::Entity::find()
            .filter(
                teams::Column::Description.contains(filter.desc_include)
            )
            .filter(
                teams::Column::Name.contains(filter.keyword)
            )
            .filter(
                teams::Column::GroupId.eq(filter.group_id)
            );
        if option.limit > 0{
            models = models.limit(option.limit as u64)
        }
        if option.offset > 0{
            models = models.offset(option.offset as u64)
        }
        for model in models.all(&self.db).await.unwrap(){
            result.push(model)
        }
        result
    }
}
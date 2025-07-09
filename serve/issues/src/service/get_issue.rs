use cert::schema::{result_error_with_msg, result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::issues;
use crate::schema::IssueFetchParam;
use crate::service::AppIssueService;

impl AppIssueService {
    pub async fn service_get_issue(&self, param: IssueFetchParam) -> AppResult<issues::Model> {
        let result = self.service_get_issue_list(param).await;
        match result.data {
            Some(data) => {
                if let Some(arr) = data.as_array() {
                    if !arr.is_empty() {
                        if let Some(first_issue) = arr.first() {
                            return match serde_json::from_value::<issues::Model>(first_issue.clone()) {
                                Ok(issue_model) => result_ok_with_data(issue_model),
                                Err(e) => result_error_with_msg_data(format!("failed to deserialize issue: {}", e)),
                            }
                        }
                    }
                }
                result_error_with_msg_data("issue not found".to_string())
            },
            None => result_error_with_msg_data("no data returned".to_string()),
        }
    }
}

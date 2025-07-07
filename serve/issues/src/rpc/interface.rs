use tarpc::service;

#[service]
pub trait IssueInterFace {
    async fn get_issue();
    async fn get_issue_list();
    async fn create_issue();
    async fn update_issue();
    
    async fn delete_issue();
    
    async fn create_label();
    async fn update_label();
    async fn link_issue_label();
    async fn unlink_issue_label();
    async fn delete_label();    
    async fn publish_comment();
    async fn update_comment();
    async fn delete_comment();
    async fn subscribe_issue();
    async fn unsubscribe_issue();
    async fn get_issue_subscribers();
    async fn update_issue_status();
    async fn update_issue_assignee();
    
}

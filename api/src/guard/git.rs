use actix_web::guard::GuardContext;
use actix_web::http::header::USER_AGENT;

pub fn git_guard(context: &GuardContext) -> bool{
    if let Some(Ok(us)) = context.head().headers.get(USER_AGENT).map(|x|x.to_str().map(|x|x.to_string())) {
        if us.starts_with("git") {
            return true;
        }
    }
    false
}
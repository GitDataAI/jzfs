pub fn user_agent_parse_os(us: Option<String>) -> Option<String> {
    if let Some(user_agent) = us.as_ref() {
        if user_agent.contains("Android") {
            Some("Android".to_string())
        } else if user_agent.contains("iPhone") {
            Some("iPhone".to_string())
        } else if user_agent.contains("iPad") {
            Some("iPad".to_string())
        } else if user_agent.contains("Windows") {
            Some("Windows".to_string())
        } else if user_agent.contains("Macintosh") {
            Some("Macintosh".to_string())
        } else {
            None
        }
    } else {
        None
    }
}

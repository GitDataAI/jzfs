use time::Duration;

#[derive(Clone, Debug)]
pub struct RusshServerConfig {
    pub port: u16,
    pub auth_rejection_time_initial: Option<Duration>,
    pub auth_rejection_time: Duration,
}

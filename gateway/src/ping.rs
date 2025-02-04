#![allow(dead_code)]


use log::{error, info, warn};
use std::time::Duration;

pub async fn find_fastest_dns_server(targets: &[&str], port: u16) -> Option<String> {
    let config = LatencyTestConfig {
        port,
        timeout: Duration::from_secs(3),
        max_retries: 2,
    };

    let tester = TcpLatencyTester::new(config);
    let results = tester.test_multiple(&targets).await;
    info!("Determining service communication IP");
    for result in &results {
        match &result.latency {
            Some(d) => info!("{}: {}ms", result.ip, d.as_millis()),
            None => warn!("{}: Failed ({})", result.ip, result.error.as_deref().unwrap_or("unknown")),
        }
    }

    if let Some(fastest) = results.fastest() {
        info!("more fast IP: {} ({}ms)",
                 fastest.ip,
                 fastest.latency.unwrap().as_millis()
        );
        return Some(fastest.ip.clone());
    } else {
        error!("All tests failed");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
     async fn test_find_fastest_ip() {
        tracing_subscriber::fmt().init();
        let ips = vec![
            "8.8.8.8",
            "1.1.1.1",
            "9.9.9.9",
        ];
        let result = find_fastest_dns_server(&ips, 443).await;
        assert!(result.is_some())
    }
}


use std::time::Instant;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::time;

#[derive(Debug, Clone)]
pub struct LatencyTestConfig {
    pub timeout: Duration,
    pub port: u16,
    pub max_retries: usize,
}

impl Default for LatencyTestConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(2),
            port: 80,
            max_retries: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LatencyResult {
    pub ip: String,
    pub port: u16,
    pub latency: Option<Duration>,
    pub error: Option<String>,
}

#[derive(Error, Debug)]
pub enum LatencyError {
    #[error("Connection timeout")]
    Timeout,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid address format")]
    InvalidAddress,
}

pub struct TcpLatencyTester {
    config: LatencyTestConfig,
}

impl TcpLatencyTester {
    pub fn new(config: LatencyTestConfig) -> Self {
        Self { config }
    }

    pub async fn test_single(&self, ip: &str) -> LatencyResult {
        let mut retries = 0;
        let port = self.config.port;

        while retries < self.config.max_retries {
            let result = self.attempt_connection(ip, port).await;

            if let Ok(latency) = result {
                return LatencyResult {
                    ip: ip.to_string(),
                    port,
                    latency: Some(latency),
                    error: None,
                };
            }

            retries += 1;
        }

        LatencyResult {
            ip: ip.to_string(),
            port,
            latency: None,
            error: Some("All retries failed".to_string()),
        }
    }

    pub async fn test_multiple(&self, ips: &[&str]) -> Vec<LatencyResult> {
        let tasks: Vec<_> = ips.iter()
            .map(|&ip| self.test_single(ip))
            .collect();

        futures::future::join_all(tasks).await
    }

    async fn attempt_connection(&self, ip: &str, port: u16) -> Result<Duration, LatencyError> {
        let addr = format!("{}:{}", ip, port);
        let start = Instant::now();

        match time::timeout(
            self.config.timeout,
            TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_)) => Ok(start.elapsed()),
            Ok(Err(e)) => Err(LatencyError::Io(e)),
            Err(_) => Err(LatencyError::Timeout),
        }
    }
}


pub trait LatencyAnalysis {
    fn fastest(&self) -> Option<&LatencyResult>;
    fn successful_results(&self) -> Vec<&LatencyResult>;
}

impl LatencyAnalysis for Vec<LatencyResult> {
    fn fastest(&self) -> Option<&LatencyResult> {
        self.iter()
            .filter_map(|r| r.latency.map(|d| (r, d)))
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(r, _)| r)
    }

    fn successful_results(&self) -> Vec<&LatencyResult> {
        self.iter()
            .filter(|r| r.latency.is_some())
            .collect()
    }
}

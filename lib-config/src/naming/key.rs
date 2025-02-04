use std::fmt::Display;
use chrono::Utc;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Clone,Debug,Eq, PartialEq)]
pub struct NamingKey {
    kind: i32,
    ips: Vec<String>,
    port: i32,
    naming: String,
    startime: i64,
}

impl NamingKey {
    pub fn default() -> Self {
        NamingKey {
            kind: 0,
            ips: sysinfo::Networks::new_with_refreshed_list()
                .iter()
                .map(|x|x.1)
                .map(|x|{
                    x.ip_networks().iter()
                        .map(|x|x.addr)
                        .filter(|x|x.is_ipv4())
                        .map(|x|x.to_string())
                        .filter(|x|!x.is_empty())
                        .collect::<Vec<String>>()
                        .join(",")
                })
                .filter(|x|!x.is_empty())
                .collect::<Vec<String>>(),
            port: 0,
            naming: "".to_string(),
            startime: Utc::now().timestamp(),
        }
    }
    pub fn set_port(&mut self, port: i32) -> &mut Self {
        self.port = port;
        self
    }
    pub fn set_naming(&mut self, naming: String) -> &mut Self {
        self.naming = naming;
        self
    }
    pub fn set_startime(&mut self, startime: i64) -> &mut Self {
        self.startime = startime;
        self
    }
    pub fn set_ips(&mut self, ips: Vec<String>) -> &mut Self {
        self.ips = ips;
        self
    }
    pub fn set_kind(&mut self, kind: i32) -> &mut Self {
        self.kind = kind;
        self
    }
}

impl Display for NamingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}:{}:{}:{}", self.kind, self.ips.join(","), self.port, self.naming, self.startime))
    }
}

impl From<String> for NamingKey {
    fn from(s: String) -> Self {
        let mut s = s.split(":");
        NamingKey {
            kind: s.next().unwrap().parse::<i32>().unwrap(),
            ips: s.next().unwrap().split(",").map(|x|x.to_string()).collect::<Vec<String>>(),
            port: s.next().unwrap().parse::<i32>().unwrap(),
            naming: s.next().unwrap().to_string(),
            startime: s.next().unwrap().parse::<i64>().unwrap(),
        }
    }
}

impl From<&str> for NamingKey {
    fn from(s: &str) -> Self {
        NamingKey::from(s.to_string())
    }
}

impl From<NamingKey> for String {
    fn from(s: NamingKey) -> Self {
        format!("{}:{}:{}:{}:{}", s.kind, s.ips.join(","), s.port, s.naming, s.startime)
    }
}

impl From<&NamingKey> for String {
    fn from(s: &NamingKey) -> Self {
        format!("{}:{}:{}:{}:{}", s.kind, s.ips.join(","), s.port, s.naming, s.startime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naming_key() {
        let mut naming_key = NamingKey::default();
        naming_key.set_port(8080);
        naming_key.set_naming("test".to_string());
        naming_key.set_startime(Utc::now().timestamp());
        naming_key.set_ips(vec!["127.0.0.1".to_string()]);
        naming_key.set_kind(0);
        println!("{}", naming_key);
        println!("{}", String::from(naming_key));
    }
}
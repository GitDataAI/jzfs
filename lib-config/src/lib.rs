#![feature(ip)]
#![allow(dead_code)]
use std::env;
use std::io;
use std::sync::Arc;
use crate::config::AppConfig;
use crate::naming::AppNaming;

pub mod naming;
pub mod config;
pub mod public;
#[derive(Clone)]
pub struct AppNacos {
    pub config: AppConfig,
    pub naming: AppNaming,
    pub endpoint: String,
}

pub struct AppNacosBuilder {
    server_name: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    namespace: String,
}

// {server_name}://{username}:{password}@{host}:{port}/{namespace}
impl AppNacos {
    pub fn parse_env() -> std::io::Result<AppNacosBuilder> {
        let env = env::var("NACOS")
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;

        let parts: Vec<&str> = env.split( "://").collect();
        let server_name = parts[0].to_string();
        let auth_and_host_port_namespace = parts[1];

        let auth_and_host_port_namespace_parts: Vec<&str> = auth_and_host_port_namespace.split('@').collect();
        if auth_and_host_port_namespace_parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid NACOS URL format (missing @)"));
        }

        let auth_parts: Vec<&str> = auth_and_host_port_namespace_parts[0].split(':').collect();
        if auth_parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid NACOS URL format (missing username:password)"));
        }

        let username = auth_parts[0].to_string();
        let password = auth_parts[1].to_string();

        let host_port_namespace = auth_and_host_port_namespace_parts[1];
        let host_port_namespace_parts: Vec<&str> = host_port_namespace.split('/').collect();
        if host_port_namespace_parts.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid NACOS URL format (missing namespace)"));
        }

        let host_port = host_port_namespace_parts[0];
        let namespace = host_port_namespace_parts[1..].join("/");

        let host_port_parts: Vec<&str> = host_port.split(':').collect();
        if host_port_parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid NACOS URL format (missing port)"));
        }

        let host = host_port_parts[0].to_string();
        let port = host_port_parts[1].parse::<u16>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port number"))?;
        Ok(AppNacosBuilder {
            server_name,
            username,
            password,
            host,
            port,
            namespace,
        })
    }
    pub fn from_env() -> io::Result<Self> {
        let AppNacosBuilder {
            server_name,
            username,
            password,
            host,
            port,
            namespace,
        } = AppNacos::parse_env()?;
        let client_props = nacos_sdk::api::props::ClientProps::new()
            .auth_username(username)
            .auth_password(password)
            .namespace(namespace)
            .max_retries(3)
            .app_name(server_name.clone())
            .remote_grpc_port(9848)
            .server_addr(format!("{}:{}", host, port));

        let naming_service = nacos_sdk::api::naming::NamingServiceBuilder::new(client_props.clone())
            .enable_auth_plugin_http()
            .build()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(client_props.clone())
            .enable_auth_plugin_http()
            .build()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        let config = AppConfig::new(Arc::new(Box::new(config_service)));
        let naming = AppNaming::new(Arc::new(Box::new(naming_service)), server_name.clone());

        Ok(Self { config, naming, endpoint: server_name})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_env() {
        unsafe {
            env::set_var("NACOS", "api://admin:admin@127.0.0.1:8848/gitdata");
        }
        let builder = AppNacos::parse_env()
            .expect("Failed to parse NACOS environment variable");
        assert_eq!(builder.server_name, "api");
        assert_eq!(builder.username, "admin");
        assert_eq!(builder.password, "admin");
        assert_eq!(builder.host, "127.0.0.1");
        assert_eq!(builder.port, 8848);
        assert_eq!(builder.namespace, "gitdata");
    }
}
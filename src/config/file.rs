use anyhow::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

use super::tls::TlsConfigFile;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub host: IpAddr,
    pub port: u16,
    pub verbose: bool,
    pub root_dir: Option<PathBuf>,
    pub tls: Option<TlsConfigFile>,
}

impl ConfigFile {
    pub fn from_file(path: Option<PathBuf>) -> Result<Self> {
        let file_path = if let Some(path) = path {
            path
        } else {
            PathBuf::from_str("server.toml").unwrap()
        };

        let file = fs::read_to_string(file_path)?;
        let config = ConfigFile::parse_toml(file.as_str())?;

        Ok(config)
    }

    fn parse_toml(content: &str) -> Result<Self> {
        match toml::from_str(content) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::msg(format!(
                "Failed to parse config from file. {}",
                err.to_string()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::config::util::tls::PrivateKeyAlgorithm;

    use super::*;

    #[test]
    fn parses_config_from_file() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878
            verbose = true
            root_dir = "~/Desktop"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let root_dir = PathBuf::from_str("~/Desktop").unwrap();
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir.unwrap(), root_dir);
        assert!(config.verbose);
    }

    #[test]
    #[should_panic(
        expected = "Failed to parse config from file. missing field `host` at line 1 column 1"
    )]
    fn checks_invalid_config_from_file() {
        let file_contents = r#"
            port = 7878
        "#;
        ConfigFile::parse_toml(file_contents).unwrap();
    }

    #[test]
    fn parses_config_with_tls_using_rsa() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878
            verbose = true
            root_dir = "~/Desktop"

            [tls]
            cert = "cert_123.pem"
            key = "key_123.pem"
            key_algorithm = "rsa"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let root_dir = PathBuf::from_str("~/Desktop").unwrap();
        let tls = TlsConfigFile {
            cert: PathBuf::from_str("cert_123.pem").unwrap(),
            key: PathBuf::from_str("key_123.pem").unwrap(),
            key_algorithm: PrivateKeyAlgorithm::Rsa,
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir.unwrap(), root_dir);
        assert_eq!(config.tls.unwrap(), tls);
        assert!(config.verbose);
    }

    #[test]
    fn parses_config_with_tls_using_pkcs8() {
        let file_contents = r#"
            host = "192.168.0.1"
            port = 7878
            verbose = true
            root_dir = "~/Desktop"

            [tls]
            cert = "cert_123.pem"
            key = "key_123.pem"
            key_algorithm = "pkcs8"
        "#;
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 7878;
        let root_dir = PathBuf::from_str("~/Desktop").unwrap();
        let tls = TlsConfigFile {
            cert: PathBuf::from_str("cert_123.pem").unwrap(),
            key: PathBuf::from_str("key_123.pem").unwrap(),
            key_algorithm: PrivateKeyAlgorithm::Pkcs8,
        };
        let config = ConfigFile::parse_toml(file_contents).unwrap();

        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.root_dir.unwrap(), root_dir);
        assert_eq!(config.tls.unwrap(), tls);
        assert!(config.verbose);
    }
}

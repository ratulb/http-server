use anyhow::{Error, Result};
use serde::Deserialize;
use std::convert::TryFrom;
use std::time::Duration;

/// CORS (Cross Origin Resource Sharing) configuration for the HTTP/S
/// server.
///
/// `CorsConfig` holds the configuration for the CORS headers for a
/// HTTP/S server instance. The following headers are supported:
///
/// Access-Control-Allow-Credentials header
/// Access-Control-Allow-Headers header
/// Access-Control-Allow-Methods header
/// Access-Control-Expose-Headers header
/// Access-Control-Max-Age header
/// Access-Control-Request-Headers header
/// Access-Control-Request-Method header
///
/// Refer to CORS here: https://www.w3.org/wiki/CORS
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorsConfig {
    /// The Access-Control-Allow-Credentials response header tells browsers
    /// whether to expose the response to frontend JavaScript code when the
    /// request's credentials mode (Request.credentials) is include.
    ///
    /// The only valid value for this header is true (case-sensitive). If you
    /// don't need credentials, omit this header entirely (rather than setting
    /// its value to false).
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Credentials
    allow_credentials: bool,
    /// The Access-Control-Allow-Headers response header is used in response to a
    /// preflight request which includes the Access-Control-Request-Headers to
    /// indicate which HTTP headers can be used during the actual request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers
    allow_headers: Option<Vec<String>>,
    /// The Access-Control-Allow-Methods response header specifies the method or
    /// methods allowed when accessing the resource in response to a preflight
    /// request.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods
    allow_methods: Option<Vec<String>>,
    /// The Access-Control-Allow-Origin response header indicates whether the
    /// response can be shared with requesting code from the given origin.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
    allow_origin: Option<String>,
    /// The Access-Control-Expose-Headers response header allows a server to
    /// indicate which response headers should be made available to scripts
    /// running in the browser, in response to a cross-origin request.
    ///
    /// Only the CORS-safelisted response headers are exposed by default.
    /// For clients to be able to access other headers, the server must list them
    /// using the Access-Control-Expose-Headers header.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers
    expose_headers: Option<Vec<String>>,
    /// The Access-Control-Max-Age response header indicates how long the results
    /// of a preflight request (that is the information contained in the
    /// Access-Control-Allow-Methods and Access-Control-Allow-Headers headers)
    /// can be cached.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Max-Age
    max_age: Option<Duration>,
    /// The Access-Control-Request-Headers request header is used by browsers
    /// when issuing a preflight request, to let the server know which HTTP
    /// headers the client might send when the actual request is made (such as
    /// with setRequestHeader()). This browser side header will be answered by
    /// the complementary server side header of Access-Control-Allow-Headers.
    ///
    /// Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Request-Headers
    request_headers: Option<Vec<String>>,
    /// The Access-Control-Request-Method request header is used by browsers when
    /// issuing a preflight request, to let the server know which HTTP method will
    /// be used when the actual request is made. This header is necessary as the
    /// preflight request is always an OPTIONS and doesn't use the same method as
    /// the actual request.
    request_method: Option<String>,
}

impl CorsConfig {
    pub fn builder() -> CorsConfigBuilder {
        CorsConfigBuilder {
            config: CorsConfig::default(),
        }
    }

    pub fn allow_all() -> Self {
        CorsConfig {
            allow_origin: Some(String::from("*")),
            allow_methods: Some(vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "PATCH".to_string(),
                "DELETE".to_string(),
                "HEAD".to_string(),
            ]),
            allow_headers: Some(vec![
                "Origin".to_string(),
                "Content-Length".to_string(),
                "Content-Type".to_string(),
            ]),
            allow_credentials: false,
            max_age: Some(Duration::from_secs(43200)),
            expose_headers: None,
            request_headers: None,
            request_method: None,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        CorsConfig {
            allow_origin: None,
            allow_methods: None,
            allow_headers: None,
            allow_credentials: false,
            max_age: None,
            expose_headers: None,
            request_headers: None,
            request_method: None,
        }
    }
}

/// CorsConfig Builder
pub struct CorsConfigBuilder {
    config: CorsConfig,
}

impl CorsConfigBuilder {
    pub fn allow_origin(mut self, origin: String) -> Self {
        self.config.allow_origin = Some(origin);
        self
    }

    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.config.allow_methods = Some(methods);
        self
    }

    pub fn allow_headers(mut self, headers: Vec<String>) -> Self {
        self.config.allow_headers = Some(headers);
        self
    }

    pub fn allow_credentials(mut self) -> Self {
        self.config.allow_credentials = true;
        self
    }

    pub fn max_age(mut self, duration: Duration) -> Self {
        self.config.max_age = Some(duration);
        self
    }

    pub fn expose_headers(mut self, headers: Vec<String>) -> Self {
        self.config.expose_headers = Some(headers);
        self
    }

    pub fn request_headers(mut self, headers: Vec<String>) -> Self {
        self.config.request_headers = Some(headers);
        self
    }

    pub fn request_method(mut self, method: String) -> Self {
        self.config.request_method = Some(method);
        self
    }

    pub fn build(self) -> CorsConfig {
        self.config
    }
}

/// CORS configuration definition for server configuration file.
/// This struct maps the values from the server configuration file
/// to a `CorsConfig` struct
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CorsConfigFile {
    pub allow_credentials: bool,
    pub allow_headers: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_origin: Option<String>,
    pub expose_headers: Option<Vec<String>>,
    pub max_age: Option<f64>,
    pub request_headers: Option<Vec<String>>,
    pub request_method: Option<String>,
}

impl TryFrom<CorsConfigFile> for CorsConfig {
    type Error = Error;

    fn try_from(file_config: CorsConfigFile) -> Result<Self, Self::Error> {
        let mut cors_config_builder = CorsConfig::builder();

        if file_config.allow_credentials {
            cors_config_builder = cors_config_builder.allow_credentials();
        }

        if let Some(allow_headers) = file_config.allow_headers {
            cors_config_builder = cors_config_builder.allow_headers(allow_headers);
        }

        if let Some(allow_methods) = file_config.allow_methods {
            cors_config_builder = cors_config_builder.allow_methods(allow_methods);
        }

        if let Some(allow_origin) = file_config.allow_origin {
            cors_config_builder = cors_config_builder.allow_origin(allow_origin);
        }

        if let Some(expose_headers) = file_config.expose_headers {
            cors_config_builder = cors_config_builder.expose_headers(expose_headers);
        }

        if let Some(max_age) = file_config.max_age {
            cors_config_builder = cors_config_builder.max_age(Duration::from_secs_f64(max_age));
        }

        if let Some(request_headers) = file_config.request_headers {
            cors_config_builder = cors_config_builder.request_headers(request_headers);
        }

        if let Some(request_method) = file_config.request_method {
            cors_config_builder = cors_config_builder.request_method(request_method);
        }

        Ok(cors_config_builder.build())
    }
}

mod tests {
    use super::*;

    #[test]
    fn creates_cors_config_with_builder() {
        let cors_config = CorsConfig::builder()
            .allow_origin("http://example.com".to_string())
            .allow_methods(vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ])
            .allow_headers(vec![
                "Content-Type".to_string(),
                "Origin".to_string(),
                "Content-Length".to_string(),
            ])
            .build();

        assert_eq!(
            cors_config.allow_origin,
            Some(String::from("http://example.com"))
        );
        assert_eq!(
            cors_config.allow_methods,
            Some(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("DELETE"),
            ])
        );
        assert_eq!(
            cors_config.allow_headers,
            Some(vec![
                String::from("Content-Type"),
                String::from("Origin"),
                String::from("Content-Length"),
            ])
        );
        assert_eq!(cors_config.allow_credentials, false);
        assert_eq!(cors_config.max_age, None);
        assert_eq!(cors_config.expose_headers, None);
        assert_eq!(cors_config.request_headers, None);
        assert_eq!(cors_config.request_method, None);
    }

    #[test]
    fn creates_cors_config_which_allows_all_connections() {
        let cors_config = CorsConfig::allow_all();

        assert_eq!(cors_config.allow_origin, Some(String::from("*")));
        assert_eq!(
            cors_config.allow_methods,
            Some(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("PATCH"),
                String::from("DELETE"),
                String::from("HEAD"),
            ])
        );
        assert_eq!(
            cors_config.allow_headers,
            Some(vec![
                String::from("Origin"),
                String::from("Content-Length"),
                String::from("Content-Type"),
            ])
        );
        assert_eq!(cors_config.allow_credentials, false);
        assert_eq!(cors_config.max_age, Some(Duration::from_secs(43200)));
        assert_eq!(cors_config.expose_headers, None);
        assert_eq!(cors_config.request_headers, None);
        assert_eq!(cors_config.request_method, None);
    }

    #[test]
    fn creates_cors_config_from_file() {
        let allow_headers = vec![
            "content-type".to_string(),
            "content-length".to_string(),
            "request-id".to_string(),
        ];
        let allow_mehtods = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
        let allow_origin = String::from("github.com");
        let expose_headers = vec!["content-type".to_string(), "request-id".to_string()];
        let max_age = 5400.;
        let request_headers = vec![
            "content-type".to_string(),
            "content-length".to_string(),
            "authorization".to_string(),
        ];
        let request_method = String::from("GET");
        let file_config = CorsConfigFile {
            allow_credentials: true,
            allow_headers: Some(allow_headers.clone()),
            allow_methods: Some(allow_mehtods.clone()),
            allow_origin: Some(allow_origin.clone()),
            expose_headers: Some(expose_headers.clone()),
            max_age: Some(max_age),
            request_headers: Some(request_headers.clone()),
            request_method: Some(request_method.clone()),
        };
        let cors_config = CorsConfig {
            allow_credentials: true,
            allow_headers: Some(allow_headers),
            allow_methods: Some(allow_mehtods),
            allow_origin: Some(allow_origin),
            expose_headers: Some(expose_headers),
            max_age: Some(Duration::from_secs_f64(max_age)),
            request_headers: Some(request_headers),
            request_method: Some(request_method),
        };

        assert_eq!(cors_config, CorsConfig::try_from(file_config).unwrap());
    }
}
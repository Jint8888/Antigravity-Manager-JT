use reqwest::{Client, Proxy};
use crate::modules::config::load_app_config;
use once_cell::sync::Lazy;

/// Global shared HTTP client (15s timeout)
/// Client has a built-in connection pool; cloning it is light and shares the pool
pub static SHARED_CLIENT: Lazy<Client> = Lazy::new(|| {
    create_base_client(15)
});

/// Global shared HTTP client (Long timeout: 60s, for warmup etc.)
pub static SHARED_CLIENT_LONG: Lazy<Client> = Lazy::new(|| {
    create_base_client(60)
});

/// Base client creation logic
fn create_base_client(timeout_secs: u64) -> Client {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs));

    if let Ok(config) = load_app_config() {
        let proxy_config = config.proxy.upstream_proxy;
        if proxy_config.enabled && !proxy_config.url.is_empty() {
            match Proxy::all(&proxy_config.url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                    tracing::info!("HTTP shared client enabled upstream proxy: {}", proxy_config.url);
                }
                Err(e) => {
                    tracing::error!("invalid_proxy_url: {}, error: {}", proxy_config.url, e);
                }
            }
        }
    }

    builder.build().unwrap_or_else(|_| Client::new())
}

/// Get uniformly configured HTTP client (15s timeout)
pub fn get_client() -> Client {
    SHARED_CLIENT.clone()
}

/// Get long timeout HTTP client (60s timeout)
pub fn get_long_client() -> Client {
    SHARED_CLIENT_LONG.clone()
}

/// Backward compatibility interface: Create uniformly configured HTTP client
pub fn create_client(timeout_secs: u64) -> Client {
    if timeout_secs == 15 {
        get_client()
    } else if timeout_secs == 60 {
        get_long_client()
    } else {
        create_base_client(timeout_secs)
    }
}

/// Create HTTP client with specific proxy configuration (special purpose)
pub fn create_client_with_proxy(
    timeout_secs: u64, 
    proxy_config: Option<crate::proxy::config::UpstreamProxyConfig>
) -> Client {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs));

    if let Some(config) = proxy_config {
        if config.enabled && !config.url.is_empty() {
            match Proxy::all(&config.url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                }
                Err(e) => {
                    tracing::error!("invalid_proxy_url: {}, error: {}", config.url, e);
                }
            }
        }
    }

    builder.build().unwrap_or_else(|_| Client::new())
}

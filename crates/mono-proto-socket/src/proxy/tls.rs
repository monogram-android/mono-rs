use crate::{ProxyConfig, ProxyError};
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::BuilderVerifierExt;
use std::net::TcpStream;
use std::sync::Arc;

pub(crate) type TlsStream = StreamOwned<ClientConnection, TcpStream>;

pub(crate) fn connect(
    stream: TcpStream,
    proxy: &ProxyConfig,
    timeout: std::time::Duration,
) -> Result<TlsStream, ProxyError> {
    let config =
        ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .and_then(|builder| builder.with_platform_verifier())
            .map_err(|error| ProxyError::Tls(format!("TLS verifier setup failed: {error}")))?
            .with_no_client_auth();
    connect_with_config(stream, proxy, timeout, config)
}

pub(crate) fn connect_with_config(
    stream: TcpStream,
    proxy: &ProxyConfig,
    timeout: std::time::Duration,
    config: ClientConfig,
) -> Result<TlsStream, ProxyError> {
    let server_name = ServerName::try_from(proxy.host.clone())
        .map_err(|_| ProxyError::Tls("HTTPS proxy host is not a valid TLS name".into()))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    let connection = ClientConnection::new(Arc::new(config), server_name)
        .map_err(|error| ProxyError::Tls(format!("TLS client setup failed: {error}")))?;
    Ok(StreamOwned::new(connection, stream))
}

pub(crate) fn http_connect(
    stream: &mut TlsStream,
    target: &str,
    proxy: &ProxyConfig,
) -> Result<(), ProxyError> {
    crate::http_connect(stream, target, proxy)
}

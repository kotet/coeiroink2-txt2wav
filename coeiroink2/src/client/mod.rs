use async_trait::async_trait;
use futures::{Stream, future, future::BoxFuture, stream, future::TryFutureExt, future::FutureExt, stream::StreamExt};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, Request, Response, service::Service, Uri};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::borrow::Cow;
use std::convert::TryInto;
use std::io::{ErrorKind, Read};
use std::error::Error;
use std::future::Future;
use std::fmt;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::task::{Context, Poll};
use swagger::{ApiError, AuthData, BodyExt, Connector, DropContextService, Has, XSpanIdString};
use url::form_urlencoded;

use mime::Mime;
use std::io::Cursor;
use multipart::client::lazy::Multipart;

use crate::models;
use crate::header;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// This encode set is used for object IDs
///
/// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
/// the vertical bar (|) is encoded.
#[allow(dead_code)]
const ID_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'|');

use crate::{Api,
     DownloadInfosV1DownloadInfosGetResponse,
     EstimateF0V1EstimateF0PostResponse,
     EstimateProsodyV1EstimateProsodyPostResponse,
     GetEngineInfoV1EngineInfoGetResponse,
     GetSampleVoiceV1SampleVoiceGetResponse,
     GetSpeakerPolicyV1SpeakerPolicyGetResponse,
     PredictV1PredictPostResponse,
     PredictV1PredictWithDurationPostResponse,
     ProcessV1ProcessPostResponse,
     ProcessV1ProcessWithPitchPostResponse,
     ReadRootGetResponse,
     SetDictionaryV1SetDictionaryPostResponse,
     SpeakerFolderPathV1Query2prosodyPostResponse,
     SpeakerFolderPathV1SpeakerFolderPathGetResponse,
     SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse,
     SpeakersV1SpeakersGetResponse,
     SynthesisV1SynthesisPostResponse
     };

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: impl TryInto<Uri, Error=hyper::http::uri::InvalidUri>, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input.try_into()?;

    let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or(ClientInitError::MissingHost)?;
    let port = uri.port_u16().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}{}", scheme, host, port, uri.path().trim_end_matches('/')))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Marker
    marker: PhantomData<fn(C)>,
}

impl<S, C> fmt::Debug for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<S, C> Clone for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Self {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
            marker: PhantomData,
        }
    }
}

impl<Connector, C> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C> where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError>
    {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
    Https(hyper::client::Client<HttpsConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
       match self {
          HyperClient::Http(client) => client.poll_ready(cx),
          HyperClient::Https(client) => client.poll_ready(cx),
       }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
       match self {
          HyperClient::Http(client) => client.call(req),
          HyperClient::Https(client) => client.call(req)
       }
    }
}

impl<C> Client<DropContextService<HyperClient, C>, C> where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path)?;

        let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => {
                HyperClient::Http(hyper::client::Client::builder().build(connector.build()))
            },
            "https" => {
                let connector = connector.https()
                   .build()
                   .map_err(ClientInitError::SslError)?;
                HyperClient::Https(hyper::client::Client::builder().build(connector))
            },
            _ => {
                return Err(ClientInitError::InvalidScheme);
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

impl<C> Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new_http(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
type HttpsConnector = hyper_openssl::HttpsConnector<hyper::client::HttpConnector>;

impl<C> Client<DropContextService<hyper::client::Client<HttpsConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError>
    {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<S, C> Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper::service::Service` /
    /// `tower::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: S,
        base_path: &str,
    ) -> Result<Self, ClientInitError>
    {
        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

#[async_trait]
impl<S, C> Api<C> for Client<S, C> where
    S: Service<
       (Request<Body>, C),
       Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Has<XSpanIdString>  + Clone + Send + Sync + 'static,
{
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), crate::ServiceError>> {
        match self.client_service.clone().poll_ready(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(o)),
            Poll::Pending => Poll::Pending,
        }
    }

    async fn download_infos_v1_download_infos_get(
        &self,
        context: &C) -> Result<DownloadInfosV1DownloadInfosGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/download_infos",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::DownloadableModel>>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadInfosV1DownloadInfosGetResponse::SuccessfulResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn estimate_f0_v1_estimate_f0_post(
        &self,
        param_wav_with_duration: models::WavWithDuration,
        context: &C) -> Result<EstimateF0V1EstimateF0PostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/estimate_f0",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_wav_with_duration).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WorldF0>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(EstimateF0V1EstimateF0PostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(EstimateF0V1EstimateF0PostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn estimate_prosody_v1_estimate_prosody_post(
        &self,
        param_prosody_making_param: models::ProsodyMakingParam,
        context: &C) -> Result<EstimateProsodyV1EstimateProsodyPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/estimate_prosody",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_prosody_making_param).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Prosody>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(EstimateProsodyV1EstimateProsodyPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(EstimateProsodyV1EstimateProsodyPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_engine_info_v1_engine_info_get(
        &self,
        context: &C) -> Result<GetEngineInfoV1EngineInfoGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/engine_info",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::EngineInfo>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetEngineInfoV1EngineInfoGetResponse::SuccessfulResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_sample_voice_v1_sample_voice_get(
        &self,
        param_speaker_uuid: Option<String>,
        param_style_id: Option<i32>,
        param_index: Option<i32>,
        context: &C) -> Result<GetSampleVoiceV1SampleVoiceGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/sample_voice",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_speaker_uuid) = param_speaker_uuid {
                query_string.append_pair("speakerUuid",
                    &param_speaker_uuid);
            }
            if let Some(param_style_id) = param_style_id {
                query_string.append_pair("styleId",
                    &param_style_id.to_string());
            }
            if let Some(param_index) = param_index {
                query_string.append_pair("index",
                    &param_index.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = swagger::ByteArray(body.to_vec());
                Ok(GetSampleVoiceV1SampleVoiceGetResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetSampleVoiceV1SampleVoiceGetResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_speaker_policy_v1_speaker_policy_get(
        &self,
        param_speaker_uuid: Option<String>,
        context: &C) -> Result<GetSpeakerPolicyV1SpeakerPolicyGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/speaker_policy",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_speaker_uuid) = param_speaker_uuid {
                query_string.append_pair("speakerUuid",
                    &param_speaker_uuid);
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SpeakerPolicy>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetSpeakerPolicyV1SpeakerPolicyGetResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetSpeakerPolicyV1SpeakerPolicyGetResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn predict_v1_predict_post(
        &self,
        param_wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/predict",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_wav_making_param).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = swagger::ByteArray(body.to_vec());
                Ok(PredictV1PredictPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(PredictV1PredictPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn predict_v1_predict_with_duration_post(
        &self,
        param_wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictWithDurationPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/predict_with_duration",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_wav_making_param).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WavWithDuration>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(PredictV1PredictWithDurationPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(PredictV1PredictWithDurationPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn process_v1_process_post(
        &self,
        param_wav: swagger::ByteArray,
        param_volume_scale: Option<f64>,
        param_pitch_scale: Option<f64>,
        param_intonation_scale: Option<f64>,
        param_pre_phoneme_length: Option<f64>,
        param_post_phoneme_length: Option<f64>,
        param_output_sampling_rate: Option<i32>,
        context: &C) -> Result<ProcessV1ProcessPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/process",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let (body_string, multipart_header) = {
            let mut multipart = Multipart::new();

            // For each parameter, encode as appropriate and add to the multipart body as a stream.

            let wav_str = match serde_json::to_string(&param_wav) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize wav to string: {}", e))),
            };

            let wav_vec = wav_str.as_bytes().to_vec();
            let wav_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let wav_cursor = Cursor::new(wav_vec);

            multipart.add_stream("wav",  wav_cursor,  None as Option<&str>, Some(wav_mime));


            let volume_scale_str = match serde_json::to_string(&param_volume_scale) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize volume_scale to string: {}", e))),
            };

            let volume_scale_vec = volume_scale_str.as_bytes().to_vec();
            let volume_scale_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let volume_scale_cursor = Cursor::new(volume_scale_vec);

            multipart.add_stream("volume_scale",  volume_scale_cursor,  None as Option<&str>, Some(volume_scale_mime));


            let pitch_scale_str = match serde_json::to_string(&param_pitch_scale) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize pitch_scale to string: {}", e))),
            };

            let pitch_scale_vec = pitch_scale_str.as_bytes().to_vec();
            let pitch_scale_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let pitch_scale_cursor = Cursor::new(pitch_scale_vec);

            multipart.add_stream("pitch_scale",  pitch_scale_cursor,  None as Option<&str>, Some(pitch_scale_mime));


            let intonation_scale_str = match serde_json::to_string(&param_intonation_scale) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize intonation_scale to string: {}", e))),
            };

            let intonation_scale_vec = intonation_scale_str.as_bytes().to_vec();
            let intonation_scale_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let intonation_scale_cursor = Cursor::new(intonation_scale_vec);

            multipart.add_stream("intonation_scale",  intonation_scale_cursor,  None as Option<&str>, Some(intonation_scale_mime));


            let pre_phoneme_length_str = match serde_json::to_string(&param_pre_phoneme_length) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize pre_phoneme_length to string: {}", e))),
            };

            let pre_phoneme_length_vec = pre_phoneme_length_str.as_bytes().to_vec();
            let pre_phoneme_length_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let pre_phoneme_length_cursor = Cursor::new(pre_phoneme_length_vec);

            multipart.add_stream("pre_phoneme_length",  pre_phoneme_length_cursor,  None as Option<&str>, Some(pre_phoneme_length_mime));


            let post_phoneme_length_str = match serde_json::to_string(&param_post_phoneme_length) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize post_phoneme_length to string: {}", e))),
            };

            let post_phoneme_length_vec = post_phoneme_length_str.as_bytes().to_vec();
            let post_phoneme_length_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let post_phoneme_length_cursor = Cursor::new(post_phoneme_length_vec);

            multipart.add_stream("post_phoneme_length",  post_phoneme_length_cursor,  None as Option<&str>, Some(post_phoneme_length_mime));


            let output_sampling_rate_str = match serde_json::to_string(&param_output_sampling_rate) {
                Ok(str) => str,
                Err(e) => return Err(ApiError(format!("Unable to serialize output_sampling_rate to string: {}", e))),
            };

            let output_sampling_rate_vec = output_sampling_rate_str.as_bytes().to_vec();
            let output_sampling_rate_mime = mime_0_2::Mime::from_str("application/json").expect("impossible to fail to parse");
            let output_sampling_rate_cursor = Cursor::new(output_sampling_rate_vec);

            multipart.add_stream("output_sampling_rate",  output_sampling_rate_cursor,  None as Option<&str>, Some(output_sampling_rate_mime));


            let mut fields = match multipart.prepare() {
                Ok(fields) => fields,
                Err(err) => return Err(ApiError(format!("Unable to build request: {}", err))),
            };

            let mut body_string = String::new();

            match fields.read_to_string(&mut body_string) {
                Ok(_) => (),
                Err(err) => return Err(ApiError(format!("Unable to build body: {}", err))),
            }

            let boundary = fields.boundary();

            let multipart_header = format!("multipart/form-data;boundary={}", boundary);

            (body_string, multipart_header)
        };

        *request.body_mut() = Body::from(body_string);

        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(&multipart_header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", multipart_header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = swagger::ByteArray(body.to_vec());
                Ok(ProcessV1ProcessPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ProcessV1ProcessPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn process_v1_process_with_pitch_post(
        &self,
        param_wav_processing_param: models::WavProcessingParam,
        context: &C) -> Result<ProcessV1ProcessWithPitchPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/process_with_pitch",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_wav_processing_param).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = swagger::ByteArray(body.to_vec());
                Ok(ProcessV1ProcessWithPitchPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ProcessV1ProcessWithPitchPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn read_root_get(
        &self,
        context: &C) -> Result<ReadRootGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ReadRootGetResponse::SuccessfulResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn set_dictionary_v1_set_dictionary_post(
        &self,
        param_dictionary_words: models::DictionaryWords,
        context: &C) -> Result<SetDictionaryV1SetDictionaryPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/set_dictionary",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_dictionary_words).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<serde_json::Value>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SetDictionaryV1SetDictionaryPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SetDictionaryV1SetDictionaryPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn speaker_folder_path_v1_query2prosody_post(
        &self,
        param_audio_query: models::AudioQuery,
        context: &C) -> Result<SpeakerFolderPathV1Query2prosodyPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/query2prosody",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_audio_query).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Prosody>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1Query2prosodyPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1Query2prosodyPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn speaker_folder_path_v1_speaker_folder_path_get(
        &self,
        param_speaker_uuid: Option<String>,
        context: &C) -> Result<SpeakerFolderPathV1SpeakerFolderPathGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/speaker_folder_path",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_speaker_uuid) = param_speaker_uuid {
                query_string.append_pair("speakerUuid",
                    &param_speaker_uuid);
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SpeakerFolderPath>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1SpeakerFolderPathGetResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1SpeakerFolderPathGetResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn speaker_folder_path_v1_style_id_to_speaker_meta_post(
        &self,
        param_style_id: Option<i32>,
        context: &C) -> Result<SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/style_id_to_speaker_meta",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_style_id) = param_style_id {
                query_string.append_pair("styleId",
                    &param_style_id.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SpeakerMetaForTextBox>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn speakers_v1_speakers_get(
        &self,
        context: &C) -> Result<SpeakersV1SpeakersGetResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/speakers",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::SpeakerMeta>>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SpeakersV1SpeakersGetResponse::SuccessfulResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn synthesis_v1_synthesis_post(
        &self,
        param_synthesis_param: models::SynthesisParam,
        context: &C) -> Result<SynthesisV1SynthesisPostResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/v1/synthesis",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_synthesis_param).expect("impossible to fail to serialize");

                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = swagger::ByteArray(body.to_vec());
                Ok(SynthesisV1SynthesisPostResponse::SuccessfulResponse
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HttpValidationError>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(SynthesisV1SynthesisPostResponse::ValidationError
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

}

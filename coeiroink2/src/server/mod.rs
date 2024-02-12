use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;
use multipart::server::Multipart;
use multipart::server::save::SaveResult;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

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

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/$",
            r"^/v1/download_infos$",
            r"^/v1/engine_info$",
            r"^/v1/estimate_f0$",
            r"^/v1/estimate_prosody$",
            r"^/v1/predict$",
            r"^/v1/predict_with_duration$",
            r"^/v1/process$",
            r"^/v1/process_with_pitch$",
            r"^/v1/query2prosody$",
            r"^/v1/sample_voice$",
            r"^/v1/set_dictionary$",
            r"^/v1/speaker_folder_path$",
            r"^/v1/speaker_policy$",
            r"^/v1/speakers$",
            r"^/v1/style_id_to_speaker_meta$",
            r"^/v1/synthesis$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_: usize = 0;
    pub(crate) static ID_V1_DOWNLOAD_INFOS: usize = 1;
    pub(crate) static ID_V1_ENGINE_INFO: usize = 2;
    pub(crate) static ID_V1_ESTIMATE_F0: usize = 3;
    pub(crate) static ID_V1_ESTIMATE_PROSODY: usize = 4;
    pub(crate) static ID_V1_PREDICT: usize = 5;
    pub(crate) static ID_V1_PREDICT_WITH_DURATION: usize = 6;
    pub(crate) static ID_V1_PROCESS: usize = 7;
    pub(crate) static ID_V1_PROCESS_WITH_PITCH: usize = 8;
    pub(crate) static ID_V1_QUERY2PROSODY: usize = 9;
    pub(crate) static ID_V1_SAMPLE_VOICE: usize = 10;
    pub(crate) static ID_V1_SET_DICTIONARY: usize = 11;
    pub(crate) static ID_V1_SPEAKER_FOLDER_PATH: usize = 12;
    pub(crate) static ID_V1_SPEAKER_POLICY: usize = 13;
    pub(crate) static ID_V1_SPEAKERS: usize = 14;
    pub(crate) static ID_V1_STYLE_ID_TO_SPEAKER_META: usize = 15;
    pub(crate) static ID_V1_SYNTHESIS: usize = 16;
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // DownloadInfosV1DownloadInfosGet - GET /v1/download_infos
            hyper::Method::GET if path.matched(paths::ID_V1_DOWNLOAD_INFOS) => {
                                let result = api_impl.download_infos_v1_download_infos_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DownloadInfosV1DownloadInfosGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DOWNLOAD_INFOS_V1_DOWNLOAD_INFOS_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // EstimateF0V1EstimateF0Post - POST /v1/estimate_f0
            hyper::Method::POST if path.matched(paths::ID_V1_ESTIMATE_F0) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_wav_with_duration: Option<models::WavWithDuration> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_wav_with_duration) => param_wav_with_duration,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WavWithDuration - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WavWithDuration due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_wav_with_duration = match param_wav_with_duration {
                                    Some(param_wav_with_duration) => param_wav_with_duration,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WavWithDuration"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WavWithDuration")),
                                };

                                let result = api_impl.estimate_f0_v1_estimate_f0_post(
                                            param_wav_with_duration,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                EstimateF0V1EstimateF0PostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ESTIMATE_F0_V1_ESTIMATE_F0_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                EstimateF0V1EstimateF0PostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ESTIMATE_F0_V1_ESTIMATE_F0_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WavWithDuration: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WavWithDuration")),
                        }
            },

            // EstimateProsodyV1EstimateProsodyPost - POST /v1/estimate_prosody
            hyper::Method::POST if path.matched(paths::ID_V1_ESTIMATE_PROSODY) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_prosody_making_param: Option<models::ProsodyMakingParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_prosody_making_param) => param_prosody_making_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ProsodyMakingParam - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ProsodyMakingParam due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_prosody_making_param = match param_prosody_making_param {
                                    Some(param_prosody_making_param) => param_prosody_making_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ProsodyMakingParam"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ProsodyMakingParam")),
                                };

                                let result = api_impl.estimate_prosody_v1_estimate_prosody_post(
                                            param_prosody_making_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                EstimateProsodyV1EstimateProsodyPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ESTIMATE_PROSODY_V1_ESTIMATE_PROSODY_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                EstimateProsodyV1EstimateProsodyPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ESTIMATE_PROSODY_V1_ESTIMATE_PROSODY_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ProsodyMakingParam: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ProsodyMakingParam")),
                        }
            },

            // GetEngineInfoV1EngineInfoGet - GET /v1/engine_info
            hyper::Method::GET if path.matched(paths::ID_V1_ENGINE_INFO) => {
                                let result = api_impl.get_engine_info_v1_engine_info_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetEngineInfoV1EngineInfoGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ENGINE_INFO_V1_ENGINE_INFO_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetSampleVoiceV1SampleVoiceGet - GET /v1/sample_voice
            hyper::Method::GET if path.matched(paths::ID_V1_SAMPLE_VOICE) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_speaker_uuid = query_params.iter().filter(|e| e.0 == "speakerUuid").map(|e| e.1.clone())
                    .next();
                let param_speaker_uuid = match param_speaker_uuid {
                    Some(param_speaker_uuid) => {
                        let param_speaker_uuid =
                            <String as std::str::FromStr>::from_str
                                (&param_speaker_uuid);
                        match param_speaker_uuid {
                            Ok(param_speaker_uuid) => Some(param_speaker_uuid),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter speakerUuid - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter speakerUuid")),
                        }
                    },
                    None => None,
                };
                let param_style_id = query_params.iter().filter(|e| e.0 == "styleId").map(|e| e.1.clone())
                    .next();
                let param_style_id = match param_style_id {
                    Some(param_style_id) => {
                        let param_style_id =
                            <i32 as std::str::FromStr>::from_str
                                (&param_style_id);
                        match param_style_id {
                            Ok(param_style_id) => Some(param_style_id),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter styleId - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter styleId")),
                        }
                    },
                    None => None,
                };
                let param_index = query_params.iter().filter(|e| e.0 == "index").map(|e| e.1.clone())
                    .next();
                let param_index = match param_index {
                    Some(param_index) => {
                        let param_index =
                            <i32 as std::str::FromStr>::from_str
                                (&param_index);
                        match param_index {
                            Ok(param_index) => Some(param_index),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter index - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter index")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_sample_voice_v1_sample_voice_get(
                                            param_speaker_uuid,
                                            param_style_id,
                                            param_index,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSampleVoiceV1SampleVoiceGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("audio/wav")
                                                            .expect("Unable to create Content-Type header for GET_SAMPLE_VOICE_V1_SAMPLE_VOICE_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = body.0;
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetSampleVoiceV1SampleVoiceGetResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SAMPLE_VOICE_V1_SAMPLE_VOICE_GET_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetSpeakerPolicyV1SpeakerPolicyGet - GET /v1/speaker_policy
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKER_POLICY) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_speaker_uuid = query_params.iter().filter(|e| e.0 == "speakerUuid").map(|e| e.1.clone())
                    .next();
                let param_speaker_uuid = match param_speaker_uuid {
                    Some(param_speaker_uuid) => {
                        let param_speaker_uuid =
                            <String as std::str::FromStr>::from_str
                                (&param_speaker_uuid);
                        match param_speaker_uuid {
                            Ok(param_speaker_uuid) => Some(param_speaker_uuid),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter speakerUuid - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter speakerUuid")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_speaker_policy_v1_speaker_policy_get(
                                            param_speaker_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSpeakerPolicyV1SpeakerPolicyGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SPEAKER_POLICY_V1_SPEAKER_POLICY_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetSpeakerPolicyV1SpeakerPolicyGetResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SPEAKER_POLICY_V1_SPEAKER_POLICY_GET_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PredictV1PredictPost - POST /v1/predict
            hyper::Method::POST if path.matched(paths::ID_V1_PREDICT) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_wav_making_param: Option<models::WavMakingParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_wav_making_param) => param_wav_making_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WavMakingParam - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WavMakingParam due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_wav_making_param = match param_wav_making_param {
                                    Some(param_wav_making_param) => param_wav_making_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WavMakingParam"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WavMakingParam")),
                                };

                                let result = api_impl.predict_v1_predict_post(
                                            param_wav_making_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PredictV1PredictPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("audio/wav")
                                                            .expect("Unable to create Content-Type header for PREDICT_V1_PREDICT_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = body.0;
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                PredictV1PredictPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PREDICT_V1_PREDICT_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WavMakingParam: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WavMakingParam")),
                        }
            },

            // PredictV1PredictWithDurationPost - POST /v1/predict_with_duration
            hyper::Method::POST if path.matched(paths::ID_V1_PREDICT_WITH_DURATION) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_wav_making_param: Option<models::WavMakingParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_wav_making_param) => param_wav_making_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WavMakingParam - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WavMakingParam due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_wav_making_param = match param_wav_making_param {
                                    Some(param_wav_making_param) => param_wav_making_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WavMakingParam"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WavMakingParam")),
                                };

                                let result = api_impl.predict_v1_predict_with_duration_post(
                                            param_wav_making_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PredictV1PredictWithDurationPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PREDICT_V1_PREDICT_WITH_DURATION_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                PredictV1PredictWithDurationPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PREDICT_V1_PREDICT_WITH_DURATION_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WavMakingParam: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WavMakingParam")),
                        }
            },

            // ProcessV1ProcessPost - POST /v1/process
            hyper::Method::POST if path.matched(paths::ID_V1_PROCESS) => {
                let boundary = match swagger::multipart::form::boundary(&headers) {
                    Some(boundary) => boundary.to_string(),
                    None => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from("Couldn't find valid multipart body".to_string()))
                                .expect("Unable to create Bad Request response for incorrect boundary")),
                };

                // Form Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw();
                match result.await {
                            Ok(body) => {
                                use std::io::Read;

                                // Read Form Parameters from body
                                let mut entries = match Multipart::with_body(&body.to_vec()[..], boundary).save().temp() {
                                    SaveResult::Full(entries) => {
                                        entries
                                    },
                                    _ => {
                                        return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Unable to process all message parts".to_string()))
                                                        .expect("Unable to create Bad Request response due to failure to process all message"))
                                    },
                                };
                                let field_wav = entries.fields.remove("wav");
                                let param_wav = match field_wav {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for wav");
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let wav_model: swagger::ByteArray = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("wav data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter wav"))
                                            }
                                        };
                                        wav_model
                                    },
                                    None => {
                                        return Ok(
                                            Response::builder()
                                            .status(StatusCode::BAD_REQUEST)
                                            .body(Body::from("Missing required form parameter wav".to_string()))
                                            .expect("Unable to create Bad Request due to missing required form parameter wav"))
                                    }
                                };
                                let field_volume_scale = entries.fields.remove("volume_scale");
                                let param_volume_scale = match field_volume_scale {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for volume_scale");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let volume_scale_model: f64 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("volume_scale data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter volume_scale"))
                                            }
                                        };
                                        volume_scale_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let field_pitch_scale = entries.fields.remove("pitch_scale");
                                let param_pitch_scale = match field_pitch_scale {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for pitch_scale");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let pitch_scale_model: f64 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("pitch_scale data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter pitch_scale"))
                                            }
                                        };
                                        pitch_scale_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let field_intonation_scale = entries.fields.remove("intonation_scale");
                                let param_intonation_scale = match field_intonation_scale {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for intonation_scale");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let intonation_scale_model: f64 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("intonation_scale data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter intonation_scale"))
                                            }
                                        };
                                        intonation_scale_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let field_pre_phoneme_length = entries.fields.remove("pre_phoneme_length");
                                let param_pre_phoneme_length = match field_pre_phoneme_length {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for pre_phoneme_length");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let pre_phoneme_length_model: f64 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("pre_phoneme_length data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter pre_phoneme_length"))
                                            }
                                        };
                                        pre_phoneme_length_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let field_post_phoneme_length = entries.fields.remove("post_phoneme_length");
                                let param_post_phoneme_length = match field_post_phoneme_length {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for post_phoneme_length");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let post_phoneme_length_model: f64 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("post_phoneme_length data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter post_phoneme_length"))
                                            }
                                        };
                                        post_phoneme_length_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let field_output_sampling_rate = entries.fields.remove("output_sampling_rate");
                                let param_output_sampling_rate = match field_output_sampling_rate {
                                    Some(field) => {
                                        let mut reader = field[0].data.readable().expect("Unable to read field for output_sampling_rate");
                                    Some({
                                        let mut data = String::new();
                                        reader.read_to_string(&mut data).expect("Reading saved String should never fail");
                                        let output_sampling_rate_model: i32 = match serde_json::from_str(&data) {
                                            Ok(model) => model,
                                            Err(e) => {
                                                return Ok(
                                                    Response::builder()
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .body(Body::from(format!("output_sampling_rate data does not match API definition : {}", e)))
                                                    .expect("Unable to create Bad Request due to missing required form parameter output_sampling_rate"))
                                            }
                                        };
                                        output_sampling_rate_model
                                    })
                                    },
                                    None => {
                                            None
                                    }
                                };
                                let result = api_impl.process_v1_process_post(
                                            param_wav,
                                            param_volume_scale,
                                            param_pitch_scale,
                                            param_intonation_scale,
                                            param_pre_phoneme_length,
                                            param_post_phoneme_length,
                                            param_output_sampling_rate,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ProcessV1ProcessPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("audio/wav")
                                                            .expect("Unable to create Content-Type header for PROCESS_V1_PROCESS_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = body.0;
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ProcessV1ProcessPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PROCESS_V1_PROCESS_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from("Couldn't read multipart body".to_string()))
                                                .expect("Unable to create Bad Request response due to unable read multipart body")),
                        }
            },

            // ProcessV1ProcessWithPitchPost - POST /v1/process_with_pitch
            hyper::Method::POST if path.matched(paths::ID_V1_PROCESS_WITH_PITCH) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_wav_processing_param: Option<models::WavProcessingParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_wav_processing_param) => param_wav_processing_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WavProcessingParam - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WavProcessingParam due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_wav_processing_param = match param_wav_processing_param {
                                    Some(param_wav_processing_param) => param_wav_processing_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WavProcessingParam"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WavProcessingParam")),
                                };

                                let result = api_impl.process_v1_process_with_pitch_post(
                                            param_wav_processing_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ProcessV1ProcessWithPitchPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("audio/wav")
                                                            .expect("Unable to create Content-Type header for PROCESS_V1_PROCESS_WITH_PITCH_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = body.0;
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ProcessV1ProcessWithPitchPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PROCESS_V1_PROCESS_WITH_PITCH_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WavProcessingParam: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WavProcessingParam")),
                        }
            },

            // ReadRootGet - GET /
            hyper::Method::GET if path.matched(paths::ID_) => {
                                let result = api_impl.read_root_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ReadRootGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for READ_ROOT_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SetDictionaryV1SetDictionaryPost - POST /v1/set_dictionary
            hyper::Method::POST if path.matched(paths::ID_V1_SET_DICTIONARY) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dictionary_words: Option<models::DictionaryWords> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dictionary_words) => param_dictionary_words,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter DictionaryWords - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter DictionaryWords due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_dictionary_words = match param_dictionary_words {
                                    Some(param_dictionary_words) => param_dictionary_words,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter DictionaryWords"))
                                                        .expect("Unable to create Bad Request response for missing body parameter DictionaryWords")),
                                };

                                let result = api_impl.set_dictionary_v1_set_dictionary_post(
                                            param_dictionary_words,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetDictionaryV1SetDictionaryPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_DICTIONARY_V1_SET_DICTIONARY_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                SetDictionaryV1SetDictionaryPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_DICTIONARY_V1_SET_DICTIONARY_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DictionaryWords: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DictionaryWords")),
                        }
            },

            // SpeakerFolderPathV1Query2prosodyPost - POST /v1/query2prosody
            hyper::Method::POST if path.matched(paths::ID_V1_QUERY2PROSODY) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_audio_query: Option<models::AudioQuery> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_audio_query) => param_audio_query,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter AudioQuery - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter AudioQuery due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_audio_query = match param_audio_query {
                                    Some(param_audio_query) => param_audio_query,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter AudioQuery"))
                                                        .expect("Unable to create Bad Request response for missing body parameter AudioQuery")),
                                };

                                let result = api_impl.speaker_folder_path_v1_query2prosody_post(
                                            param_audio_query,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SpeakerFolderPathV1Query2prosodyPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_QUERY2PROSODY_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                SpeakerFolderPathV1Query2prosodyPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_QUERY2PROSODY_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AudioQuery: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AudioQuery")),
                        }
            },

            // SpeakerFolderPathV1SpeakerFolderPathGet - GET /v1/speaker_folder_path
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKER_FOLDER_PATH) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_speaker_uuid = query_params.iter().filter(|e| e.0 == "speakerUuid").map(|e| e.1.clone())
                    .next();
                let param_speaker_uuid = match param_speaker_uuid {
                    Some(param_speaker_uuid) => {
                        let param_speaker_uuid =
                            <String as std::str::FromStr>::from_str
                                (&param_speaker_uuid);
                        match param_speaker_uuid {
                            Ok(param_speaker_uuid) => Some(param_speaker_uuid),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter speakerUuid - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter speakerUuid")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.speaker_folder_path_v1_speaker_folder_path_get(
                                            param_speaker_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SpeakerFolderPathV1SpeakerFolderPathGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_SPEAKER_FOLDER_PATH_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                SpeakerFolderPathV1SpeakerFolderPathGetResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_SPEAKER_FOLDER_PATH_GET_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SpeakerFolderPathV1StyleIdToSpeakerMetaPost - POST /v1/style_id_to_speaker_meta
            hyper::Method::POST if path.matched(paths::ID_V1_STYLE_ID_TO_SPEAKER_META) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_style_id = query_params.iter().filter(|e| e.0 == "styleId").map(|e| e.1.clone())
                    .next();
                let param_style_id = match param_style_id {
                    Some(param_style_id) => {
                        let param_style_id =
                            <i32 as std::str::FromStr>::from_str
                                (&param_style_id);
                        match param_style_id {
                            Ok(param_style_id) => Some(param_style_id),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter styleId - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter styleId")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.speaker_folder_path_v1_style_id_to_speaker_meta_post(
                                            param_style_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_STYLE_ID_TO_SPEAKER_META_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKER_FOLDER_PATH_V1_STYLE_ID_TO_SPEAKER_META_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SpeakersV1SpeakersGet - GET /v1/speakers
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKERS) => {
                                let result = api_impl.speakers_v1_speakers_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SpeakersV1SpeakersGetResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SPEAKERS_V1_SPEAKERS_GET_SUCCESSFUL_RESPONSE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SynthesisV1SynthesisPost - POST /v1/synthesis
            hyper::Method::POST if path.matched(paths::ID_V1_SYNTHESIS) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_synthesis_param: Option<models::SynthesisParam> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_synthesis_param) => param_synthesis_param,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SynthesisParam - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SynthesisParam due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_synthesis_param = match param_synthesis_param {
                                    Some(param_synthesis_param) => param_synthesis_param,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SynthesisParam"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SynthesisParam")),
                                };

                                let result = api_impl.synthesis_v1_synthesis_post(
                                            param_synthesis_param,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SynthesisV1SynthesisPostResponse::SuccessfulResponse
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("audio/wav")
                                                            .expect("Unable to create Content-Type header for SYNTHESIS_V1_SYNTHESIS_POST_SUCCESSFUL_RESPONSE"));
                                                    let body_content = body.0;
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                SynthesisV1SynthesisPostResponse::ValidationError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SYNTHESIS_V1_SYNTHESIS_POST_VALIDATION_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SynthesisParam: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SynthesisParam")),
                        }
            },

            _ if path.matched(paths::ID_) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_DOWNLOAD_INFOS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_ENGINE_INFO) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_ESTIMATE_F0) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_ESTIMATE_PROSODY) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREDICT) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREDICT_WITH_DURATION) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PROCESS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PROCESS_WITH_PITCH) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_QUERY2PROSODY) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SAMPLE_VOICE) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SET_DICTIONARY) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SPEAKER_FOLDER_PATH) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SPEAKER_POLICY) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SPEAKERS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_STYLE_ID_TO_SPEAKER_META) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_SYNTHESIS) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // DownloadInfosV1DownloadInfosGet - GET /v1/download_infos
            hyper::Method::GET if path.matched(paths::ID_V1_DOWNLOAD_INFOS) => Some("DownloadInfosV1DownloadInfosGet"),
            // EstimateF0V1EstimateF0Post - POST /v1/estimate_f0
            hyper::Method::POST if path.matched(paths::ID_V1_ESTIMATE_F0) => Some("EstimateF0V1EstimateF0Post"),
            // EstimateProsodyV1EstimateProsodyPost - POST /v1/estimate_prosody
            hyper::Method::POST if path.matched(paths::ID_V1_ESTIMATE_PROSODY) => Some("EstimateProsodyV1EstimateProsodyPost"),
            // GetEngineInfoV1EngineInfoGet - GET /v1/engine_info
            hyper::Method::GET if path.matched(paths::ID_V1_ENGINE_INFO) => Some("GetEngineInfoV1EngineInfoGet"),
            // GetSampleVoiceV1SampleVoiceGet - GET /v1/sample_voice
            hyper::Method::GET if path.matched(paths::ID_V1_SAMPLE_VOICE) => Some("GetSampleVoiceV1SampleVoiceGet"),
            // GetSpeakerPolicyV1SpeakerPolicyGet - GET /v1/speaker_policy
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKER_POLICY) => Some("GetSpeakerPolicyV1SpeakerPolicyGet"),
            // PredictV1PredictPost - POST /v1/predict
            hyper::Method::POST if path.matched(paths::ID_V1_PREDICT) => Some("PredictV1PredictPost"),
            // PredictV1PredictWithDurationPost - POST /v1/predict_with_duration
            hyper::Method::POST if path.matched(paths::ID_V1_PREDICT_WITH_DURATION) => Some("PredictV1PredictWithDurationPost"),
            // ProcessV1ProcessPost - POST /v1/process
            hyper::Method::POST if path.matched(paths::ID_V1_PROCESS) => Some("ProcessV1ProcessPost"),
            // ProcessV1ProcessWithPitchPost - POST /v1/process_with_pitch
            hyper::Method::POST if path.matched(paths::ID_V1_PROCESS_WITH_PITCH) => Some("ProcessV1ProcessWithPitchPost"),
            // ReadRootGet - GET /
            hyper::Method::GET if path.matched(paths::ID_) => Some("ReadRootGet"),
            // SetDictionaryV1SetDictionaryPost - POST /v1/set_dictionary
            hyper::Method::POST if path.matched(paths::ID_V1_SET_DICTIONARY) => Some("SetDictionaryV1SetDictionaryPost"),
            // SpeakerFolderPathV1Query2prosodyPost - POST /v1/query2prosody
            hyper::Method::POST if path.matched(paths::ID_V1_QUERY2PROSODY) => Some("SpeakerFolderPathV1Query2prosodyPost"),
            // SpeakerFolderPathV1SpeakerFolderPathGet - GET /v1/speaker_folder_path
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKER_FOLDER_PATH) => Some("SpeakerFolderPathV1SpeakerFolderPathGet"),
            // SpeakerFolderPathV1StyleIdToSpeakerMetaPost - POST /v1/style_id_to_speaker_meta
            hyper::Method::POST if path.matched(paths::ID_V1_STYLE_ID_TO_SPEAKER_META) => Some("SpeakerFolderPathV1StyleIdToSpeakerMetaPost"),
            // SpeakersV1SpeakersGet - GET /v1/speakers
            hyper::Method::GET if path.matched(paths::ID_V1_SPEAKERS) => Some("SpeakersV1SpeakersGet"),
            // SynthesisV1SynthesisPost - POST /v1/synthesis
            hyper::Method::POST if path.matched(paths::ID_V1_SYNTHESIS) => Some("SynthesisV1SynthesisPost"),
            _ => None,
        }
    }
}

//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
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
    SynthesisV1SynthesisPostResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Download Infos
    async fn download_infos_v1_download_infos_get(
        &self,
        context: &C) -> Result<DownloadInfosV1DownloadInfosGetResponse, ApiError>
    {
        info!("download_infos_v1_download_infos_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Estimate F0
    async fn estimate_f0_v1_estimate_f0_post(
        &self,
        wav_with_duration: models::WavWithDuration,
        context: &C) -> Result<EstimateF0V1EstimateF0PostResponse, ApiError>
    {
        info!("estimate_f0_v1_estimate_f0_post({:?}) - X-Span-ID: {:?}", wav_with_duration, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Estimate Prosody
    async fn estimate_prosody_v1_estimate_prosody_post(
        &self,
        prosody_making_param: models::ProsodyMakingParam,
        context: &C) -> Result<EstimateProsodyV1EstimateProsodyPostResponse, ApiError>
    {
        info!("estimate_prosody_v1_estimate_prosody_post({:?}) - X-Span-ID: {:?}", prosody_making_param, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Engine Info
    async fn get_engine_info_v1_engine_info_get(
        &self,
        context: &C) -> Result<GetEngineInfoV1EngineInfoGetResponse, ApiError>
    {
        info!("get_engine_info_v1_engine_info_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Sample Voice
    async fn get_sample_voice_v1_sample_voice_get(
        &self,
        speaker_uuid: Option<String>,
        style_id: Option<i32>,
        index: Option<i32>,
        context: &C) -> Result<GetSampleVoiceV1SampleVoiceGetResponse, ApiError>
    {
        info!("get_sample_voice_v1_sample_voice_get({:?}, {:?}, {:?}) - X-Span-ID: {:?}", speaker_uuid, style_id, index, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Speaker Policy
    async fn get_speaker_policy_v1_speaker_policy_get(
        &self,
        speaker_uuid: Option<String>,
        context: &C) -> Result<GetSpeakerPolicyV1SpeakerPolicyGetResponse, ApiError>
    {
        info!("get_speaker_policy_v1_speaker_policy_get({:?}) - X-Span-ID: {:?}", speaker_uuid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Predict
    async fn predict_v1_predict_post(
        &self,
        wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictPostResponse, ApiError>
    {
        info!("predict_v1_predict_post({:?}) - X-Span-ID: {:?}", wav_making_param, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Predict
    async fn predict_v1_predict_with_duration_post(
        &self,
        wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictWithDurationPostResponse, ApiError>
    {
        info!("predict_v1_predict_with_duration_post({:?}) - X-Span-ID: {:?}", wav_making_param, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Process
    async fn process_v1_process_post(
        &self,
        wav: swagger::ByteArray,
        volume_scale: Option<f64>,
        pitch_scale: Option<f64>,
        intonation_scale: Option<f64>,
        pre_phoneme_length: Option<f64>,
        post_phoneme_length: Option<f64>,
        output_sampling_rate: Option<i32>,
        context: &C) -> Result<ProcessV1ProcessPostResponse, ApiError>
    {
        info!("process_v1_process_post({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", wav, volume_scale, pitch_scale, intonation_scale, pre_phoneme_length, post_phoneme_length, output_sampling_rate, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Process
    async fn process_v1_process_with_pitch_post(
        &self,
        wav_processing_param: models::WavProcessingParam,
        context: &C) -> Result<ProcessV1ProcessWithPitchPostResponse, ApiError>
    {
        info!("process_v1_process_with_pitch_post({:?}) - X-Span-ID: {:?}", wav_processing_param, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Read Root
    async fn read_root_get(
        &self,
        context: &C) -> Result<ReadRootGetResponse, ApiError>
    {
        info!("read_root_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Set Dictionary
    async fn set_dictionary_v1_set_dictionary_post(
        &self,
        dictionary_words: models::DictionaryWords,
        context: &C) -> Result<SetDictionaryV1SetDictionaryPostResponse, ApiError>
    {
        info!("set_dictionary_v1_set_dictionary_post({:?}) - X-Span-ID: {:?}", dictionary_words, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_query2prosody_post(
        &self,
        audio_query: models::AudioQuery,
        context: &C) -> Result<SpeakerFolderPathV1Query2prosodyPostResponse, ApiError>
    {
        info!("speaker_folder_path_v1_query2prosody_post({:?}) - X-Span-ID: {:?}", audio_query, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_speaker_folder_path_get(
        &self,
        speaker_uuid: Option<String>,
        context: &C) -> Result<SpeakerFolderPathV1SpeakerFolderPathGetResponse, ApiError>
    {
        info!("speaker_folder_path_v1_speaker_folder_path_get({:?}) - X-Span-ID: {:?}", speaker_uuid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_style_id_to_speaker_meta_post(
        &self,
        style_id: Option<i32>,
        context: &C) -> Result<SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse, ApiError>
    {
        info!("speaker_folder_path_v1_style_id_to_speaker_meta_post({:?}) - X-Span-ID: {:?}", style_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Speakers
    async fn speakers_v1_speakers_get(
        &self,
        context: &C) -> Result<SpeakersV1SpeakersGetResponse, ApiError>
    {
        info!("speakers_v1_speakers_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Synthesis
    async fn synthesis_v1_synthesis_post(
        &self,
        synthesis_param: models::SynthesisParam,
        context: &C) -> Result<SynthesisV1SynthesisPostResponse, ApiError>
    {
        info!("synthesis_v1_synthesis_post({:?}) - X-Span-ID: {:?}", synthesis_param, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}

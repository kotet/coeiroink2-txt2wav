#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DownloadInfosV1DownloadInfosGetResponse {
    /// Successful Response
    SuccessfulResponse
    (Vec<models::DownloadableModel>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum EstimateF0V1EstimateF0PostResponse {
    /// Successful Response
    SuccessfulResponse
    (models::WorldF0)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum EstimateProsodyV1EstimateProsodyPostResponse {
    /// Successful Response
    SuccessfulResponse
    (models::Prosody)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetEngineInfoV1EngineInfoGetResponse {
    /// Successful Response
    SuccessfulResponse
    (models::EngineInfo)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetSampleVoiceV1SampleVoiceGetResponse {
    /// Successful Response
    SuccessfulResponse
    (swagger::ByteArray)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetSpeakerPolicyV1SpeakerPolicyGetResponse {
    /// Successful Response
    SuccessfulResponse
    (models::SpeakerPolicy)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PredictV1PredictPostResponse {
    /// Successful Response
    SuccessfulResponse
    (swagger::ByteArray)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PredictV1PredictWithDurationPostResponse {
    /// Successful Response
    SuccessfulResponse
    (models::WavWithDuration)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ProcessV1ProcessPostResponse {
    /// Successful Response
    SuccessfulResponse
    (swagger::ByteArray)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ProcessV1ProcessWithPitchPostResponse {
    /// Successful Response
    SuccessfulResponse
    (swagger::ByteArray)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReadRootGetResponse {
    /// Successful Response
    SuccessfulResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SetDictionaryV1SetDictionaryPostResponse {
    /// Successful Response
    SuccessfulResponse
    (serde_json::Value)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SpeakerFolderPathV1Query2prosodyPostResponse {
    /// Successful Response
    SuccessfulResponse
    (models::Prosody)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SpeakerFolderPathV1SpeakerFolderPathGetResponse {
    /// Successful Response
    SuccessfulResponse
    (models::SpeakerFolderPath)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse {
    /// Successful Response
    SuccessfulResponse
    (models::SpeakerMetaForTextBox)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SpeakersV1SpeakersGetResponse {
    /// Successful Response
    SuccessfulResponse
    (Vec<models::SpeakerMeta>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum SynthesisV1SynthesisPostResponse {
    /// Successful Response
    SuccessfulResponse
    (swagger::ByteArray)
    ,
    /// Validation Error
    ValidationError
    (models::HttpValidationError)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Download Infos
    async fn download_infos_v1_download_infos_get(
        &self,
        context: &C) -> Result<DownloadInfosV1DownloadInfosGetResponse, ApiError>;

    /// Estimate F0
    async fn estimate_f0_v1_estimate_f0_post(
        &self,
        wav_with_duration: models::WavWithDuration,
        context: &C) -> Result<EstimateF0V1EstimateF0PostResponse, ApiError>;

    /// Estimate Prosody
    async fn estimate_prosody_v1_estimate_prosody_post(
        &self,
        prosody_making_param: models::ProsodyMakingParam,
        context: &C) -> Result<EstimateProsodyV1EstimateProsodyPostResponse, ApiError>;

    /// Get Engine Info
    async fn get_engine_info_v1_engine_info_get(
        &self,
        context: &C) -> Result<GetEngineInfoV1EngineInfoGetResponse, ApiError>;

    /// Get Sample Voice
    async fn get_sample_voice_v1_sample_voice_get(
        &self,
        speaker_uuid: Option<String>,
        style_id: Option<i32>,
        index: Option<i32>,
        context: &C) -> Result<GetSampleVoiceV1SampleVoiceGetResponse, ApiError>;

    /// Get Speaker Policy
    async fn get_speaker_policy_v1_speaker_policy_get(
        &self,
        speaker_uuid: Option<String>,
        context: &C) -> Result<GetSpeakerPolicyV1SpeakerPolicyGetResponse, ApiError>;

    /// Predict
    async fn predict_v1_predict_post(
        &self,
        wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictPostResponse, ApiError>;

    /// Predict
    async fn predict_v1_predict_with_duration_post(
        &self,
        wav_making_param: models::WavMakingParam,
        context: &C) -> Result<PredictV1PredictWithDurationPostResponse, ApiError>;

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
        context: &C) -> Result<ProcessV1ProcessPostResponse, ApiError>;

    /// Process
    async fn process_v1_process_with_pitch_post(
        &self,
        wav_processing_param: models::WavProcessingParam,
        context: &C) -> Result<ProcessV1ProcessWithPitchPostResponse, ApiError>;

    /// Read Root
    async fn read_root_get(
        &self,
        context: &C) -> Result<ReadRootGetResponse, ApiError>;

    /// Set Dictionary
    async fn set_dictionary_v1_set_dictionary_post(
        &self,
        dictionary_words: models::DictionaryWords,
        context: &C) -> Result<SetDictionaryV1SetDictionaryPostResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_query2prosody_post(
        &self,
        audio_query: models::AudioQuery,
        context: &C) -> Result<SpeakerFolderPathV1Query2prosodyPostResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_speaker_folder_path_get(
        &self,
        speaker_uuid: Option<String>,
        context: &C) -> Result<SpeakerFolderPathV1SpeakerFolderPathGetResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_style_id_to_speaker_meta_post(
        &self,
        style_id: Option<i32>,
        context: &C) -> Result<SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse, ApiError>;

    /// Speakers
    async fn speakers_v1_speakers_get(
        &self,
        context: &C) -> Result<SpeakersV1SpeakersGetResponse, ApiError>;

    /// Synthesis
    async fn synthesis_v1_synthesis_post(
        &self,
        synthesis_param: models::SynthesisParam,
        context: &C) -> Result<SynthesisV1SynthesisPostResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Download Infos
    async fn download_infos_v1_download_infos_get(
        &self,
        ) -> Result<DownloadInfosV1DownloadInfosGetResponse, ApiError>;

    /// Estimate F0
    async fn estimate_f0_v1_estimate_f0_post(
        &self,
        wav_with_duration: models::WavWithDuration,
        ) -> Result<EstimateF0V1EstimateF0PostResponse, ApiError>;

    /// Estimate Prosody
    async fn estimate_prosody_v1_estimate_prosody_post(
        &self,
        prosody_making_param: models::ProsodyMakingParam,
        ) -> Result<EstimateProsodyV1EstimateProsodyPostResponse, ApiError>;

    /// Get Engine Info
    async fn get_engine_info_v1_engine_info_get(
        &self,
        ) -> Result<GetEngineInfoV1EngineInfoGetResponse, ApiError>;

    /// Get Sample Voice
    async fn get_sample_voice_v1_sample_voice_get(
        &self,
        speaker_uuid: Option<String>,
        style_id: Option<i32>,
        index: Option<i32>,
        ) -> Result<GetSampleVoiceV1SampleVoiceGetResponse, ApiError>;

    /// Get Speaker Policy
    async fn get_speaker_policy_v1_speaker_policy_get(
        &self,
        speaker_uuid: Option<String>,
        ) -> Result<GetSpeakerPolicyV1SpeakerPolicyGetResponse, ApiError>;

    /// Predict
    async fn predict_v1_predict_post(
        &self,
        wav_making_param: models::WavMakingParam,
        ) -> Result<PredictV1PredictPostResponse, ApiError>;

    /// Predict
    async fn predict_v1_predict_with_duration_post(
        &self,
        wav_making_param: models::WavMakingParam,
        ) -> Result<PredictV1PredictWithDurationPostResponse, ApiError>;

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
        ) -> Result<ProcessV1ProcessPostResponse, ApiError>;

    /// Process
    async fn process_v1_process_with_pitch_post(
        &self,
        wav_processing_param: models::WavProcessingParam,
        ) -> Result<ProcessV1ProcessWithPitchPostResponse, ApiError>;

    /// Read Root
    async fn read_root_get(
        &self,
        ) -> Result<ReadRootGetResponse, ApiError>;

    /// Set Dictionary
    async fn set_dictionary_v1_set_dictionary_post(
        &self,
        dictionary_words: models::DictionaryWords,
        ) -> Result<SetDictionaryV1SetDictionaryPostResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_query2prosody_post(
        &self,
        audio_query: models::AudioQuery,
        ) -> Result<SpeakerFolderPathV1Query2prosodyPostResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_speaker_folder_path_get(
        &self,
        speaker_uuid: Option<String>,
        ) -> Result<SpeakerFolderPathV1SpeakerFolderPathGetResponse, ApiError>;

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_style_id_to_speaker_meta_post(
        &self,
        style_id: Option<i32>,
        ) -> Result<SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse, ApiError>;

    /// Speakers
    async fn speakers_v1_speakers_get(
        &self,
        ) -> Result<SpeakersV1SpeakersGetResponse, ApiError>;

    /// Synthesis
    async fn synthesis_v1_synthesis_post(
        &self,
        synthesis_param: models::SynthesisParam,
        ) -> Result<SynthesisV1SynthesisPostResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Download Infos
    async fn download_infos_v1_download_infos_get(
        &self,
        ) -> Result<DownloadInfosV1DownloadInfosGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().download_infos_v1_download_infos_get(&context).await
    }

    /// Estimate F0
    async fn estimate_f0_v1_estimate_f0_post(
        &self,
        wav_with_duration: models::WavWithDuration,
        ) -> Result<EstimateF0V1EstimateF0PostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().estimate_f0_v1_estimate_f0_post(wav_with_duration, &context).await
    }

    /// Estimate Prosody
    async fn estimate_prosody_v1_estimate_prosody_post(
        &self,
        prosody_making_param: models::ProsodyMakingParam,
        ) -> Result<EstimateProsodyV1EstimateProsodyPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().estimate_prosody_v1_estimate_prosody_post(prosody_making_param, &context).await
    }

    /// Get Engine Info
    async fn get_engine_info_v1_engine_info_get(
        &self,
        ) -> Result<GetEngineInfoV1EngineInfoGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_engine_info_v1_engine_info_get(&context).await
    }

    /// Get Sample Voice
    async fn get_sample_voice_v1_sample_voice_get(
        &self,
        speaker_uuid: Option<String>,
        style_id: Option<i32>,
        index: Option<i32>,
        ) -> Result<GetSampleVoiceV1SampleVoiceGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_sample_voice_v1_sample_voice_get(speaker_uuid, style_id, index, &context).await
    }

    /// Get Speaker Policy
    async fn get_speaker_policy_v1_speaker_policy_get(
        &self,
        speaker_uuid: Option<String>,
        ) -> Result<GetSpeakerPolicyV1SpeakerPolicyGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_speaker_policy_v1_speaker_policy_get(speaker_uuid, &context).await
    }

    /// Predict
    async fn predict_v1_predict_post(
        &self,
        wav_making_param: models::WavMakingParam,
        ) -> Result<PredictV1PredictPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().predict_v1_predict_post(wav_making_param, &context).await
    }

    /// Predict
    async fn predict_v1_predict_with_duration_post(
        &self,
        wav_making_param: models::WavMakingParam,
        ) -> Result<PredictV1PredictWithDurationPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().predict_v1_predict_with_duration_post(wav_making_param, &context).await
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
        ) -> Result<ProcessV1ProcessPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().process_v1_process_post(wav, volume_scale, pitch_scale, intonation_scale, pre_phoneme_length, post_phoneme_length, output_sampling_rate, &context).await
    }

    /// Process
    async fn process_v1_process_with_pitch_post(
        &self,
        wav_processing_param: models::WavProcessingParam,
        ) -> Result<ProcessV1ProcessWithPitchPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().process_v1_process_with_pitch_post(wav_processing_param, &context).await
    }

    /// Read Root
    async fn read_root_get(
        &self,
        ) -> Result<ReadRootGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().read_root_get(&context).await
    }

    /// Set Dictionary
    async fn set_dictionary_v1_set_dictionary_post(
        &self,
        dictionary_words: models::DictionaryWords,
        ) -> Result<SetDictionaryV1SetDictionaryPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().set_dictionary_v1_set_dictionary_post(dictionary_words, &context).await
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_query2prosody_post(
        &self,
        audio_query: models::AudioQuery,
        ) -> Result<SpeakerFolderPathV1Query2prosodyPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().speaker_folder_path_v1_query2prosody_post(audio_query, &context).await
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_speaker_folder_path_get(
        &self,
        speaker_uuid: Option<String>,
        ) -> Result<SpeakerFolderPathV1SpeakerFolderPathGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().speaker_folder_path_v1_speaker_folder_path_get(speaker_uuid, &context).await
    }

    /// Speaker Folder Path
    async fn speaker_folder_path_v1_style_id_to_speaker_meta_post(
        &self,
        style_id: Option<i32>,
        ) -> Result<SpeakerFolderPathV1StyleIdToSpeakerMetaPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().speaker_folder_path_v1_style_id_to_speaker_meta_post(style_id, &context).await
    }

    /// Speakers
    async fn speakers_v1_speakers_get(
        &self,
        ) -> Result<SpeakersV1SpeakersGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().speakers_v1_speakers_get(&context).await
    }

    /// Synthesis
    async fn synthesis_v1_synthesis_post(
        &self,
        synthesis_param: models::SynthesisParam,
        ) -> Result<SynthesisV1SynthesisPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().synthesis_v1_synthesis_post(synthesis_param, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;

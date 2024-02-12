#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use openapi_client::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "DownloadInfosV1DownloadInfosGet",
                "GetEngineInfoV1EngineInfoGet",
                "GetSampleVoiceV1SampleVoiceGet",
                "GetSpeakerPolicyV1SpeakerPolicyGet",
                "ProcessV1ProcessPost",
                "ReadRootGet",
                "SpeakerFolderPathV1SpeakerFolderPathGet",
                "SpeakerFolderPathV1StyleIdToSpeakerMetaPost",
                "SpeakersV1SpeakersGet",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("DownloadInfosV1DownloadInfosGet") => {
            let result = rt.block_on(client.download_infos_v1_download_infos_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("EstimateF0V1EstimateF0Post") => {
            let result = rt.block_on(client.estimate_f0_v1_estimate_f0_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("EstimateProsodyV1EstimateProsodyPost") => {
            let result = rt.block_on(client.estimate_prosody_v1_estimate_prosody_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("GetEngineInfoV1EngineInfoGet") => {
            let result = rt.block_on(client.get_engine_info_v1_engine_info_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSampleVoiceV1SampleVoiceGet") => {
            let result = rt.block_on(client.get_sample_voice_v1_sample_voice_get(
                  Some("speaker_uuid_example".to_string()),
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSpeakerPolicyV1SpeakerPolicyGet") => {
            let result = rt.block_on(client.get_speaker_policy_v1_speaker_policy_get(
                  Some("speaker_uuid_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("PredictV1PredictPost") => {
            let result = rt.block_on(client.predict_v1_predict_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("PredictV1PredictWithDurationPost") => {
            let result = rt.block_on(client.predict_v1_predict_with_duration_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("ProcessV1ProcessPost") => {
            let result = rt.block_on(client.process_v1_process_post(
                  swagger::ByteArray(Vec::from("BINARY_DATA_HERE")),
                  Some(8.14),
                  Some(8.14),
                  Some(8.14),
                  Some(8.14),
                  Some(8.14),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("ProcessV1ProcessWithPitchPost") => {
            let result = rt.block_on(client.process_v1_process_with_pitch_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("ReadRootGet") => {
            let result = rt.block_on(client.read_root_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetDictionaryV1SetDictionaryPost") => {
            let result = rt.block_on(client.set_dictionary_v1_set_dictionary_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("SpeakerFolderPathV1Query2prosodyPost") => {
            let result = rt.block_on(client.speaker_folder_path_v1_query2prosody_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("SpeakerFolderPathV1SpeakerFolderPathGet") => {
            let result = rt.block_on(client.speaker_folder_path_v1_speaker_folder_path_get(
                  Some("speaker_uuid_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SpeakerFolderPathV1StyleIdToSpeakerMetaPost") => {
            let result = rt.block_on(client.speaker_folder_path_v1_style_id_to_speaker_meta_post(
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SpeakersV1SpeakersGet") => {
            let result = rt.block_on(client.speakers_v1_speakers_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SynthesisV1SynthesisPost") => {
            let result = rt.block_on(client.synthesis_v1_synthesis_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        _ => {
            panic!("Invalid operation provided")
        }
    }
}

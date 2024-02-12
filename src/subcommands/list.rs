use crate::typedef::*;
use clap::Args;

use serde::Serialize;
use serde_json;

#[derive(Debug, Args)]
pub struct ListCommandArgs {
    #[arg(short, long, help = "output as json")]
    json: bool,
}

#[derive(Serialize, Debug)]
struct Speaker {
    name: String,
    uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    styles: Vec<Style>,
}

#[derive(Serialize, Debug)]
struct Style {
    name: String,
    id: i32,
}

pub fn list_command(
    args: ListCommandArgs,
    client: &Box<dyn coeiroink2::ApiNoContext<ClientContext>>,
    runtime: &tokio::runtime::Runtime,
) {
    let coeiroink2::SpeakersV1SpeakersGetResponse::SuccessfulResponse(response) = runtime
        .block_on(client.speakers_v1_speakers_get())
        .expect("failed to retrieve speaker info");

    let mut speakers = vec![];
    for sm in response {
        speakers.push(Speaker {
            name: sm.speaker_name,
            uuid: sm.speaker_uuid,
            version: sm.version,
            styles: sm.styles.iter().map(|x| Style {
                name: x.style_name.clone(),
                id: x.style_id,
            }).collect()
        });
    }

    if args.json {
        let serialized = serde_json::to_string(&speakers).expect("failed to serialize to json");
        println!("{}", serialized);
        return;
    }

    for speaker in speakers {
        println!("{}: {}", speaker.name, speaker.uuid);
        for style in speaker.styles {
            println!("\t{}: {}", style.name, style.id);
        }
    }
}

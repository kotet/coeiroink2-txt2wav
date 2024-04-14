use crate::typedef::*;
use coeiroink2::models::WavMakingParam;

use clap::{Args, ValueEnum};
use hound;
use indicatif;

use regex;

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Auto,
    Wav,
}

const DELIMITERS: &[char] = &[
    '\n', '\t', '、', '。', ',', '.', '!', '?', '！', '？', '「', '」',
];

#[derive(Debug, Args)]
pub struct PredictCommandArgs {
    #[arg(short, long, help = "input text")]
    input: String,
    #[arg(short, long, help = "output file", default_value = "output.wav")]
    output: String,
    #[arg(
        short,
        long,
        value_enum,
        help = "output format (by default, auto-detect from file extension)",
        default_value = "auto"
    )]
    format: OutputFormat,
    #[arg(
        short = 'u',
        long,
        help = "speaker uuid",
        default_value = "3c37646f-3881-5374-2a83-149267990abc"
    )]
    speaker_id: String,
    #[arg(short, long, help = "style id", default_value = "0")]
    style: i32,
    #[arg(short = 'r', long, help = "speed scale", default_value = "1.0")]
    speed: f64,
    #[arg(long, help = "channel count", default_value = "1")]
    channels: u16,
    #[arg(long, help = "sample rate", default_value = "44100")]
    sample_rate: u32,
    #[arg(long, help = "bits per sample", default_value = "16")]
    bits_per_sample: u16,
}

pub fn predict_command(
    args: PredictCommandArgs,
    client: &Box<dyn coeiroink2::ApiNoContext<ClientContext>>,
    runtime: &tokio::runtime::Runtime,
) {
    let text = std::fs::read_to_string(&args.input).expect("failed to read input file");

    let text = pre_process(&text);

    let text_length = text.chars().count();

    let texts = split_text(&text);

    let mut dst = hound::WavWriter::create(
        args.output,
        hound::WavSpec {
            channels: args.channels,
            sample_rate: args.sample_rate,
            bits_per_sample: args.bits_per_sample,
            sample_format: hound::SampleFormat::Int,
        },
    )
    .expect("failed to create wav file");

    let pb = indicatif::ProgressBar::new(text_length as u64);

    pb.set_style(
        indicatif::ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("=> "),
    );

    for text in &texts {
        let (cur, text) = text;
        pb.set_position(*cur as u64);
        pb.set_message(format!("predicting: {:?}", text).clone());
        let response = runtime.block_on(client.predict_v1_predict_post(WavMakingParam {
            speaker_uuid: args.speaker_id.clone(),
            style_id: args.style,
            text: text.to_string(),
            prosody_detail: None,
            speed_scale: args.speed,
        }));
        if let Err(err) = response {
            pb.println(format!(
                "failed to predict speech for {:?}: {:?}",
                text, err
            ));
            continue;
        }
        let response = response.unwrap();
        if let coeiroink2::PredictV1PredictPostResponse::ValidationError(err) = response {
            eprintln!("validation error");
            if let Some(detail) = err.detail {
                for e in detail {
                    eprintln!("\t{}", e.msg);
                }
            }
        } else if let coeiroink2::PredictV1PredictPostResponse::SuccessfulResponse(result) =
            response
        {
            let swagger::ByteArray(result) = result;
            let mut reader =
                hound::WavReader::new(std::io::Cursor::new(result)).expect("failed to read wav");
            if dst.spec() != reader.spec() {
                eprintln!("warning: output format is different from input format");
                eprint!("output format: {:?}", dst.spec());
                eprintln!("input format: {:?}", reader.spec());
            }
            let s = reader.samples::<i16>();
            for sample in s {
                dst.write_sample(sample.unwrap())
                    .expect("failed to write sample");
            }
        } else {
            eprintln!("unknown response");
        }
    }
    pb.finish_with_message("done");
}

fn pre_process(text: &str) -> String {
    // 10,000 -> 10000
    let text = regex::Regex::new(r"[0-9]+,[0-9,]+").unwrap().replace_all(
        &text,
        |caps: &regex::Captures| {
            let mut s = caps.get(0).unwrap().as_str().to_string();
            s.retain(|c| c != ',');
            println!("replacing: {:?} -> {:?}", caps.get(0).unwrap().as_str(), s);
            s
        },
    );

    // 1~2, 1～2 -> 1から2, 1から2
    let text = regex::Regex::new(r"([0-9]+)[~～]([0-9]+)")
        .unwrap()
        .replace_all(&text, |caps: &regex::Captures| {
            let s = format!("{}から{}", &caps[1], &caps[2]);
            println!("replacing: {:?} -> {:?}", caps.get(0).unwrap().as_str(), s);
            s
        });

    // ─ (罫線) -> 。
    let text = regex::Regex::new(r"─+")
        .unwrap()
        .replace_all(&text, |caps: &regex::Captures| {
            println!(
                "replacing: {:?} -> {:?}",
                caps.get(0).unwrap().as_str(),
                "。"
            );
            "。"
        });

    text.to_string()
}

fn split_text(text: &str) -> Vec<(usize, String)> {
    let mut texts = vec![];
    let mut cur = 0;
    let mut buf = String::new();

    for (i, c) in text.chars().into_iter().enumerate() {
        // do not split if the delimiter is a part of a number
        if c == '.' {
            let next = text.chars().nth(i + 1).unwrap_or('x');
            let prev = text.chars().nth(i - 1).unwrap_or('x');
            if next.is_numeric() && prev.is_numeric() {
                buf.push(c);
                continue;
            }
        }
        if DELIMITERS.contains(&c) {
            if !buf.trim().is_empty() {
                buf.push(c);
                texts.push((cur, buf.clone().trim().to_string()));
                buf.clear();
                cur = i + 1;
            }
            continue;
        }
        buf.push(c);
    }

    texts
}

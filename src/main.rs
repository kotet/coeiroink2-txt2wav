mod subcommands;
mod typedef;
use typedef::*;

use coeiroink2::{self, ContextWrapperExt};

use clap::{Parser, Subcommand};
use swagger::{AuthData, EmptyContext, Push, XSpanIdString};
use tokio;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "list speakers and styles")]
    List(subcommands::list::ListCommandArgs),
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("failed to build tokio runtime");
    let context: ClientContext = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        None as Option<AuthData>,
        XSpanIdString::default(),
    );
    let client = Box::new(
        coeiroink2::Client::try_new_http("http://localhost:50032")
            .expect("failed to create https client"),
    );
    let client: Box<dyn coeiroink2::ApiNoContext<ClientContext>> =
        Box::new(client.with_context(context));

    let cli = Cli::parse();
    match cli.command {
        Commands::List(args) => {
            subcommands::list::list_command(args, &client, &runtime);
        }
    }
}

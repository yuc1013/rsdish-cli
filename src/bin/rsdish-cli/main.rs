mod cmd;

use clap::Parser;
use cmd::root::RootCmd;
use tracing_subscriber::FmtSubscriber;

use crate::cmd::root::handle_root;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    let cli = RootCmd::parse();
    println!("{:#?}", cli);
    handle_root(cli);
}

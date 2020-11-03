use anyhow::Result;
use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("nyaa")
        .version("1.0")
        .arg(Arg::with_name("id").required(true))
        .get_matches();

    let id = matches.value_of("id").unwrap();

    let config = nyaa::get_info(id).await?;

    nyaa::download(&config).await?;

    Ok(())
}

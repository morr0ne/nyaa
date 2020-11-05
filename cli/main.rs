use anyhow::Result;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("nyaa")
        .version("1.0")
        .arg(Arg::with_name("id").required(true))
        .get_matches();

    let id = matches.value_of("id").unwrap();

    let config = nyaa_core::get_info(id).await?;

    nyaa_core::download(&config).await?;

    Ok(())
}

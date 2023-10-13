use rurtex::{self, error::Result};
use tracing::{self, info};
use tracing_appender;
mod message;

fn main() -> Result<()>{

    let id = ulid::Ulid::new().to_string();
    let file_appender = tracing_appender::rolling::daily("/home/kasperw/logs", format!("rurtex{id}.log"));
    let (non_bloking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_bloking)
        .with_env_filter("rurtex=debug")
        .init();

    asdfsd asdf asdf asdf asd ,
    asdf 
    info!("Rurtex initialize new app with id:{id}");
    let app = rurtex::Rurtex::new();

    app.execute()?;

    Ok(())
}

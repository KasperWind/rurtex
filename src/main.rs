use rurtex::{self, error::Result};
mod message;

fn main() -> Result<()>{

    let app = rurtex::Rurtex::new();

    app.execute()?;

    Ok(())
}

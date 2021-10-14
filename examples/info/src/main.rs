use anyhow::Result;
use nano::Nano;

#[tokio::main]
async fn main() -> Result<()> {
    // let db = Nano::new("http://dev:dev@localhost:5984");
    let db = Nano::new("http://dev:dev@nano_couchdb:5984");
    let db_info = db.info().await?;
    println!("{:?}", db_info);
    Ok(())
}

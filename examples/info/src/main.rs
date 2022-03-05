use anyhow::Result;
use nano::Nano;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // let db = Nano::new("http://dev:dev@localhost:5984");
    // couchdb instance url including username and password
    let db = Nano::new("http://dev:dev@nano_couchdb:5984");
    // get couchdb informations
    let db_info = db.info().await?;
    println!("CouchDB info: {}", db_info.to_string_pretty()?);
    // create a database
    let create_db_res = db.create("bob", false).await?;
    println!("{}", create_db_res.to_string_pretty()?);
    // use bob database in order to perform document actions
    let bob = db.use_db("bob");
    // create a document on bob
    let doc_body = json!({
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let doc_created_res = bob.insert(&doc_body, Some("billy_doc"), None).await?;
    println!("Doc created: {}", doc_created_res.to_string_pretty()?);
    // delete a doc
    // get the id and rev from previously created doc, in order to delete a doc we must provide the id and rev
    let id_doc_to_destory = doc_created_res.id;
    let rev_doc_to_destory = doc_created_res.rev;
    // delete the previously created document
    let doc_deleted_res = bob.destroy(&id_doc_to_destory, &rev_doc_to_destory).await?;
    println!("Doc deleted: {}", doc_deleted_res.to_string_pretty()?);
    Ok(())
}

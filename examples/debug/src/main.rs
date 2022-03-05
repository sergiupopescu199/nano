use anyhow::Result;
use nano::Nano;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // let db = Nano::new("http://dev:dev@localhost:5984");
    // couchdb instance url including username and password
    let db = Nano::new("http://dev:dev@nano_couchdb:5984");
    // use bob database in order to perform document actions

    let j = json!({
        "conflicts": true,
        "limit": 1
        // "bella": "gente_json"
    });

    let mut url = format!("{}/{}?", "we", "bell");

    let keys = vec![
        "conflicts",
        "descending",
        "end_key",
        "endkey_docid",
        "end_key_doc_id",
        "include_docs",
        "inclusive_end",
        "key",
        "keys",
        "limit",
        "skip",
        "startkey",
        "startkey_docid",
        "start_key_doc_id",
        "update_seq",
    ];

    // for i in keys {
    //     if j[i].as_i64().is_some() {
    //         url.push_str(&*format!("{}={}&", i, j[&i].as_i64().unwrap()));
    //     }
    //     if j[i].as_str().is_some() {
    //         url.push_str(&*format!("{}={}&", i, j[i].as_str().unwrap()));
    //     }
    //     if j[i].as_bool().is_some() {
    //         url.push_str(&*format!("{}={}&", i, j[i].as_bool().unwrap()));
    //     }
    // }

    let h: Vec<_> = keys
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if j[i].as_i64().is_some() {
                println!("{}", i);
                format!("{}={}&", i, j[i].as_i64().unwrap())
            } else if j[i].as_str().is_some() {
                format!("{}={}&", i, j[i].as_str().unwrap())
            } else if j[i].as_bool().is_some() {
                format!("{}={}&", i, j[i].as_bool().unwrap())
            } else {
                format!("nope")
            }
        })
        .collect();
    println!("{:?}", h);
    url.truncate(url.len() - 1);

    println!("{}", url);

    Ok(())
}

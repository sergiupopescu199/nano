# Nano 

This library provides the ability to interact with CouchDB trough the REST API. On Nodejs the nano module is the official CouchDB driver so I wanted to create a nano like crate for Rust.

### Add crate to your project

```toml
[dependencies]
nano = { git = "https://github.com/sergiupopescu199/nano.git", branch = "master" }
```

| :point_up: Warning                                           |
| ------------------------------------------------------------ |
| This is my first library, it needs a lot of work to do. For now it will be only available on **Github**. When the functionalities will be almost on par with what the Nodejs nano module offers only then it will be published on **Crates.io**. |

### Usage

The usage is very similar to what nodejs nano module offers, I tried to use also the same methods names to make the transition very easy.
First of all we need to import some additional crates.

```toml
[dependencies]
anyhow = "1.0.44"
serde_json = "1.0.68"
nano = { git = "https://github.com/sergiupopescu199/nano.git", branch = "master" }
tokio = { version = "1.12.0", features = ["full"] }
```

 Let’s create a db and try to create and delete a document.

```rust
use anyhow::Result;
use nano::Nano;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // couchdb instance url including username and password
    let db = Nano::new("http://dev:dev@localhost:5984");
    // get couchdb informations
    let db_info = db.info().await?;
    println!("{:?}", db_info);
    // create a database
    let create_db_res = db.create("bob").await?;
    println!("{:?}", create_db_res);
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
    println!("{:?}", doc_created_res);
    // delete a doc
    // get the id and rev from previously created doc, in order to delete a doc we must provide the id and rev
    let id_doc_to_destory = doc_created_res.id;
    let rev_doc_to_destory = doc_created_res.rev;
    // delete the previously created document
    let doc_deleted_res = bob.destroy(&id_doc_to_destory, &rev_doc_to_destory).await?;
    println!("{:?}", doc_deleted_res);
    Ok(())
}
```

### Development on VsCode

This repository uses `.devcontainer` inside you can spin a couchdb instance suing `docker-compose`  so there is no need to create a separate docker-compose file

```bash
docker-compose -f .devcontainer/couchdb.yaml up -d
```

in this way you’ll be able to test how nano is interacting with the local couchdb instance by using `http://dev:dev@localhost5984`.

But the most powerful thing about **devcontainer** is the ability to develop inside a container just install this extension `ms-vscode-remote.remote-containers` and then use `Ctrl+P` and type `Remote-Containers: Open Folder in Container` and it should automatically create the rust environment


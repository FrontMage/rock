#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate serde_json;

#[cfg(test)]
mod tests;

use std::io;
use std::path::{Path, PathBuf};

use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use rocket::response::content;
use rocket::response::NamedFile;

// Global mongodb client
static mut MC: Option<Client> = None;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// Insert document into 'test.movies' collection
// coll.insert_one(doc.clone(), None)
//     .ok()
//     .expect("Failed to insert document.");

#[get("/blog_posts")]
fn get_blog_posts() -> content::Json<String> {
    unsafe {
        match MC {
            Some(ref client) => {
                let coll = client.db("blog").collection("posts");

                let doc = doc! {
                    "title": "Hello Github",
                    "body":"Golang 相关推荐项目",
                    "url":"https://gist.github.com/FrontMage/6b54b9711cdf4656c42c4973b09b7167",
                };

                // Find the document and receive a cursor
                // TODO: error handle
                let cursor = coll.find(Some(doc.clone()), None)
                    .ok()
                    .expect("Failed to execute find.");

                let mut contents = Vec::new();

                for item in cursor {
                    match item {
                        Ok(doc) => if let Ok(j) = serde_json::ser::to_string(&doc) {
                            contents.push(j);
                        },
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }

                // TODO: stripe the _id field
                content::Json(format!("[{}]", contents.join(",")))
            }
            None => content::Json(String::from("[]")),
        }
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./static").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/static", routes![files])
        .mount("/", routes![index, get_blog_posts])
}

fn main() {
    unsafe {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
        MC = Some(client);
    }
    rocket().launch();
}

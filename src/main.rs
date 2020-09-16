#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use std::io;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html")
}


#[get("/<file..>")]
fn build_dir(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).ok()
}

#[get("/static/<file..>", rank = 2)]
fn static_dir(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index,build_dir])
        .mount("/static", routes![static_dir])
}

fn main() {
    rocket().launch();
}

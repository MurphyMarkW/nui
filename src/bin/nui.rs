#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate clap;
use clap::App;

#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let yaml = load_yaml!("nui.yml");
    let matches = App::from_yaml(yaml).get_matches();

    rocket::ignite()
           .mount("/api", routes![index])
           .mount("/", StaticFiles::from("./ui/dist/"))
           .launch();
}

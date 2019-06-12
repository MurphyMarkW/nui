#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;

extern crate nui;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/version")]
fn version() -> String {
    let nmap = nui::nmap::Nmap {
        name: "test",
        hosts: vec![],
    };

    nmap.version()
}

#[get("/help")]
fn help() -> String {
    let nmap = nui::nmap::Nmap {
        name: "test",
        hosts: vec![],
    };

    nmap.help()
}

#[get("/scan?<host>")]
fn scan(host: Option<String>) -> String {
    let nmap = nui::nmap::Nmap {
        name: "test",
        hosts: vec![host.unwrap_or_default()],
    };

    nmap.scan()
}

fn main() {
    let yaml = load_yaml!("nui.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    rocket::ignite()
           .mount("/api", routes![index, scan, version, help])
           .mount("/", StaticFiles::from("./ui/dist/"))
           .launch();
}

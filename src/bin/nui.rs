#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use] extern crate rocket;
use rocket::State;
use rocket_contrib::serve::StaticFiles;

extern crate nui;
use nui::nmap::Nmap;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/version")]
fn version(nmap: State<Nmap>) -> String {
    nmap.version()
}

#[get("/help")]
fn help(nmap: State<Nmap>) -> String {
    nmap.help()
}

#[get("/scan?<host>")]
fn scan(nmap: State<Nmap>, host: Option<String>) -> String {
    nmap.scan(host.unwrap_or_default())
}

fn main() {
    let yaml = load_yaml!("nui.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    rocket::ignite()
           .mount("/api", routes![index, scan, version, help])
           .mount("/", StaticFiles::from("./ui/dist/"))
           .manage(Nmap { name: "test", hosts: vec![] })
           .launch();
}

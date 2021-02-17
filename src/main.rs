#![feature(proc_macro_hygiene, decl_macro)]

mod data;

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket::http::RawStr;
use chrono::{NaiveDate, Datelike};
use std::path::PathBuf;
use rocket::response::content::Html;

const INDEX: &str = include_str!("index.html");

#[derive(Debug)]
pub enum OuestError {
    IO(std::io::Error),
    Serde(toml::de::Error),
    UndefinedLocation,
}

impl From<std::io::Error> for OuestError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<toml::de::Error> for OuestError {
    fn from(e: toml::de::Error) -> Self {
        Self::Serde(e)
    }
}

fn format_date(date: Option<NaiveDate>) -> String {
    let date = match date {
        Some(date) => date,
        None => return "???".into(),
    };

    format!(
        "{} {}",
        date.day(),
        match date.month() {
            1 => "jan",
            2 => "fev",
            3 => "mar",
            4 => "avr",
            5 => "mai",
            6 => "juin",
            7 => "juil",
            8 => "août",
            9 => "sep",
            10 => "oct",
            11 => "nov",
            12 => "dec",
            _ => unimplemented!("unknown month number"),
        },
    )
}

#[get("/")]
fn index() -> Result<Html<String>, rocket::response::Debug<OuestError>> {
    match data::now() {
        Ok(None) => Ok(Html("No current event".into())),
        Ok(Some((loc, img, a, b))) => {
            let index = String::from(INDEX)
                .replace("{{CITY}}", &loc)
                .replace("{{A}}", &format_date(Some(a)))
                .replace("{{B}}", &format_date(b))
                .replace("{{IMAGE}}", &img)
                ;

            Ok(Html(index))
        },
        Err(e) => Err(rocket::response::Debug(e)),
    }
}

#[get("/<image>")]
fn image(image: &RawStr) -> Option<NamedFile> {
    let path = PathBuf::from(format!("data/{}", image.as_str()));
    NamedFile::open(path).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, image]).launch();
}

#![deny(warnings)]
#![forbid(unsafe_code)]

use chrono::{DateTime, Duration, Utc};
use chrono_tz::{Australia::Melbourne, Tz};
use clap::{App, Arg};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::Path;
use uuid::Uuid;
use warp::{Filter, Reply};

#[derive(Serialize)]
struct Response {
    uuid: uuid::Uuid,
    message: String,
    status: String,
    result: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Request {
    offset: Option<i64>,
    return_type: Option<String>,
}

fn char_to_brainfuck(c: char, last_value: u8) -> (String, u8) {
    let value = c as u8;

    let mut string = String::new();

    if value as i32 - last_value as i32 > 0 {
        for _ in last_value..value {
            string.push('+');
        }
    } else {
        for _ in (value..last_value).rev() {
            string.push('-');
        }
    }
    // Print
    string.push('.');

    (string, value)
}

fn string_to_brainfuck(value: &str) -> String {
    let mut string = String::new();
    let mut last_value: u8 = 0;

    for c in value.chars() {
        let result = char_to_brainfuck(c, last_value);
        string.push_str(&result.0);
        last_value = result.1;
    }

    string
}

fn time_in_4_hours(offset: i64) -> DateTime<Tz> {
    Utc::now().with_timezone(&Melbourne) + Duration::hours(4) + Duration::hours(offset * 6)
}

enum ReturnType {
    Json,
    Brainfuck,
}

struct Args {
    port: u16,
    web_dir: String,
}

#[derive(Debug)]
struct ArgError {
    details: String,
}

impl ArgError {
    fn new(msg: &str) -> ArgError {
        ArgError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ArgError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn parse_args() -> Result<Args, ArgError> {
    let matches = App::new("tgtb")
        .version("1.0")
        .author("Paul S. <paul@sarda.dev>")
        .about("Tells time in Melbourne Victoria Australia.")
        .arg(
            Arg::with_name("web-dir")
                .short("w")
                .long("web-dir")
                .required(true)
                .value_name("DIRECTORY")
                .help("Sets the location of the static web files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .required(false)
                .default_value("3030")
                .value_name("INT")
                .help("the port the warp will serve on")
                .takes_value(true),
        )
        .get_matches();

    let web_dir = matches.value_of("web-dir").unwrap();

    if !Path::new(web_dir).exists() {
        return Err(ArgError::new("Given web-dir does not exist"));
    }

    let port_str = matches.value_of("port").unwrap();
    let port = match port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_) => 0,
    };
    if port == 0 {
        return Err(ArgError::new("Cannot read given port"));
    }

    let result = Args {
        port: port,
        web_dir: web_dir.to_owned(),
    };

    Ok(result)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /api/v1/time_in_4_hours
    let time_in_4_hours = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("time_in_4_hours"))
        .and(warp::query::<Request>())
        .map(|p: Request| {
            let mut rng = rand::thread_rng();

            let result: Option<String>;
            let message: String;
            let status: String;

            let x: f64 = rng.gen(); // generates a float between 0 and 1
            if x > 0.99 {
                message = "unable to calculate time in four hours".to_owned();
                status = "failure".to_owned();
                result = Default::default();
            } else {
                let offset: i64;
                match p.offset {
                    None => offset = 0,
                    Some(request_offset) => {
                        offset = request_offset;
                    }
                }
                message = "result".to_owned();
                status = "success".to_owned();
                result = Option::from(time_in_4_hours(offset).to_rfc3339().to_owned());
            }

            let return_type = match p.return_type {
                None => ReturnType::Json,
                Some(return_type) => match return_type.to_lowercase().as_ref() {
                    "json" => ReturnType::Json,
                    "brainfuck" => ReturnType::Brainfuck,
                    _ => ReturnType::Json,
                },
            };
            match return_type {
                ReturnType::Json => {
                    let resp = Response {
                        uuid: Uuid::new_v4(),
                        message: message.to_owned(),
                        status: status.to_owned(),
                        result: result,
                    };
                    warp::reply::json(&resp).into_response()
                }
                ReturnType::Brainfuck => {
                    let string = match result {
                        None => "massive failure".to_owned(),
                        Some(value) => value,
                    };
                    warp::reply::html(string_to_brainfuck(&string).to_owned()).into_response()
                }
            }
        });

    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            panic!("{}", error.details.to_owned());
        }
    };

    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);
    let api = warp::get().and(time_in_4_hours).with(cors);
    let routes = api.or(warp::fs::dir(args.web_dir));
    print!("Running server\n");
    warp::serve(routes).run(([0, 0, 0, 0], args.port)).await;
}

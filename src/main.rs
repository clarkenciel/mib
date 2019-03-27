use chrono::prelude::*;
use uuid::Uuid;
use clap::{Arg, App, SubCommand};
use warp::{self, Filter};
use serde::{Deserialize,Serialize};

fn main() {
    let matches = App::new("mib")
        .subcommand(SubCommand::with_name("server")
                    .about("web server")
                    .arg(Arg::with_name("port")
                         .short("p")
                         .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("server") {
        let port = matches
            .value_of("port")
            .expect("Port must be provided");
        let port = u16::from_str_radix(port, 10)
            .expect(&*format!("{} is not a valid port format", port));

        let notes = warp::path("notes");
        let notes_index = notes.and(warp::path::end());
        let notes_by_id = notes.and(warp::path::param::<Uuid>()).and(warp::path::end());

        let list = warp::get2()
            .and(notes_index)
            .and_then(list_notes);

        let fetch = warp::get2()
            .and(notes_by_id)
            .and_then(fetch_note);

        let api = list.or(fetch);
        let routes = api.with(warp::log("mib"));

        warp::serve(routes).run(([0,0,0,0], port));
    } else {
        println!("{}", matches.usage());
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    note_id: Uuid,
    publication_time: DateTime<Utc>,
    content: String,
    initial_location: Point,
    current_location: Point,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point(f64, f64);

fn fetch_note(id: Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&None::<Option<Note>>))
}

fn list_notes() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&Vec::<Vec<Note>>::new()))
}

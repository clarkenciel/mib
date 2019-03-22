use chrono::prelude::*;
use geo::Point;
use uuid::Uuid;

fn main() {
    println!("Hello, world!");
}

struct Note {
    note_id: Uuid,
    publication_time: DateTime<Utc>,
    content: String,
    initial_location: Point<f64>,
    current_location: Point<f64>,
}

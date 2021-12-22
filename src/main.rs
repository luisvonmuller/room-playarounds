#![allow(dead_code, unused_mut, unused_variables)]
use std::sync::{Arc, Mutex};

mod tests;

mod room;
use room::*;

type Rooms = Arc<Mutex<Vec<Room>>>;

fn main() {
    /* Inits the Arc Mutex */
    let mut rooms_vec = Vec::<Room>::new();
    assert_eq!(rooms_vec.len(), 0); // Just to get compiler now the type 👀

    let rooms: Rooms = Arc::new(Mutex::new(rooms_vec)); // 0 size vector
}

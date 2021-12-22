#![allow(dead_code, unused_mut, unused_variables)]
use std::sync::{Arc, Mutex};

mod tests;

mod room;
use room::*;
type Rooms = Arc<Mutex<Vec<Room>>>;

fn main() {
    /* Inits the Arc Mutex */
    let mut rooms_vec = Vec::<Room>::new();
    assert_eq!(rooms_vec.len(), 0); // Just to get compiler knowing the type 👀

    use crate::tests::test_helper::generate_arc_mutex_room_vector;
    use crate::tests::test_helper::generate_semple_room;

    let mut rooms: Rooms = generate_arc_mutex_room_vector();
    let sample_room: Room = generate_semple_room();
    rooms.lock().unwrap().push(sample_room.clone());

    assert_eq!(&rooms.lock().unwrap()[0], &sample_room);
    println!("{:#?}", &rooms.lock().unwrap().len());
    let _killing = drop(&mut rooms.lock().unwrap()[0]);
    println!("{:#?}", &rooms.lock().unwrap().len());
    println!("{:#?}", &rooms.lock().unwrap()[0]);
}

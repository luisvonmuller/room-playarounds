#![allow(dead_code, unused_mut, unused_variables)]
#![feature(mutex_unlock)]
use std::sync::{Arc, Mutex};

mod room;
mod tests;
use room::*;
type Rooms = Arc<Mutex<Vec<Box<Room>>>>;

fn main() {
    /* Inits the Arc Mutex */
    let mut rooms_vec = Vec::<Box<Room>>::new();
    assert_eq!(rooms_vec.len(), 0); // Just to get compiler knowing the type 👀

    use crate::tests::test_helper::generate_arc_mutex_room_vector;
    use crate::tests::test_helper::generate_semple_room;

    let mut rooms: Rooms = generate_arc_mutex_room_vector();
    let sample_room: Room = generate_semple_room();
    rooms.lock().unwrap().push(Box::new(sample_room.clone()));
    ////////////////////////////////////////////////////////////////
    if let Ok(mut rooms_mutex_guard) = rooms.try_lock() {
        // println!("Mutex Guard Defer Content: {:#?}", *rooms_mutex_guard); // Here I have the Vec<Box<T>>
        // println!("Smart Pointer (Box): {:#?}", *rooms_mutex_guard[0]); // Here I have the "T" (unwraps)Box<T>
        let room_to_drop = &mut rooms_mutex_guard[0];
        drop(&mut **room_to_drop);
        Mutex::unlock(rooms_mutex_guard);
    };
    //drop(&mut *rooms.lock().unwrap()[0]);
    ////////////////////////////////////////////////////////////////

    println!("Rooms inside the Vec: {:#?}", rooms.lock().unwrap().len());
}

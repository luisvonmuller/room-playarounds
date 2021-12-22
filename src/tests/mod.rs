#![allow(unused_imports)]
use super::*;
use crate::peer::Peer;
pub mod test_helper;
use test_helper::*;

#[test]
fn zero_size_rooms_vec() {
    let mut rooms_vec = Vec::<Room>::new();
    assert_eq!(rooms_vec.len(), 0); // Just to get compiler now the type 👀
}

#[test]
fn zero_size_rooms_vec_on_arc_mutex() {
    let mut rooms_vec = Vec::<Room>::new();
    let rooms: Rooms = Arc::new(Mutex::new(rooms_vec)); // 0 size vector
    assert_eq!(rooms.lock().unwrap().len(), 0);
}

#[test]
fn create_room() {
    let sample_room: Room = generate_semple_room();

    let peer_one: Peer = Peer {
        user_name: "luisvonmuller".to_string(),
        origin: vec![127, 0, 0, 1],
    };

    let peer_two: Peer = Peer {
        user_name: "someone".to_string(),
        origin: vec![127, 0, 0, 1],
    };

    let _sample_room = Room {
        peers: vec![peer_one, peer_two],
        id: 123_u64,
    };

    assert_eq!(sample_room, _sample_room);
}

#[test]
fn create_a_room_into_acr_mutex() {
    let mut rooms: Rooms = generate_arc_mutex_room_vector();
    let sample_room: Room = generate_semple_room();
    rooms.lock().unwrap().push(sample_room.clone());

    assert_eq!(&rooms.lock().unwrap()[0], &sample_room);
}
#[test]
fn kill_a_room_on_arc_mutex() {
    let mut rooms: Rooms = generate_arc_mutex_room_vector();
    let sample_room: Room = generate_semple_room();
    rooms.lock().unwrap().push(sample_room.clone());

    assert_eq!(&rooms.lock().unwrap()[0], &sample_room);
    let _killing = rooms.lock().unwrap()[0].kill();
    println!("{:#?}", rooms.lock().unwrap());
    //assert_eq!(rooms.lock().unwrap().len(), 0);
}

#![allow(unused_imports)]

use super::*;
use crate::peer::Peer;

pub fn generate_arc_mutex_room_vector() -> Rooms {
    let mut rooms_vec = Vec::<Box<Room>>::new();
    let rooms: Rooms = Arc::new(Mutex::new(rooms_vec)); // 0 size vector
    assert_eq!(rooms.lock().unwrap().len(), 0);
    rooms
}

pub fn generate_semple_room() -> Room {
    let peer_one: Peer = Peer {
        user_name: "luisvonmuller".to_string(),
        origin: vec![127, 0, 0, 1],
    };

    let peer_two: Peer = Peer {
        user_name: "someone".to_string(),
        origin: vec![127, 0, 0, 1],
    };

    Room {
        peers: vec![peer_one, peer_two],
        id: 123_u64,
    }
}

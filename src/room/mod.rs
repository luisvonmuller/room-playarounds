pub mod peer;
use peer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub peers: Vec<Peer>,
    pub id: u64, // room_id:  Room Id (Temporary)
}

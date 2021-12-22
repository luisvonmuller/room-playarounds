pub mod peer;
use peer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub peers: Vec<Peer>,
    pub id: u64,
}

// impl Room {
//     pub fn kill(&mut self) -> Result<(), ()> {
//         drop(self);
//         Ok(())
//     }
// }

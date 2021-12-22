#[derive(Debug, PartialEq, Clone,)]
pub struct Peer {
    pub user_name: String,
    pub origin: Vec<u8>, // Should I even get the Peer IP? Should I not? I do't know
}

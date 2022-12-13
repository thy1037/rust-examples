#![allow(unused)]


use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::SwarmEvent,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};

///
/// ```
/// let local_key = identity::Keypair::generate_ed25519();
/// let local_peer_id = PeerId::from(local_key.public());
/// assert!(true)
/// ```
/// 
#[test]
fn new_peer_id() {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    assert!(true)
}

use common::entity::EntityType;
use core_protocol::id::GameId;
use serde::{Serialize, Deserialize};

fn main() {
	unsafe {
		EntityType::init();
	}

	let test = core_protocol::rpc::ClientRequest::CreateSession{game_id: GameId::Mk48, invitation_id: None, referrer: None, saved_session_tuple: None};

	println!("test = {:?}", test);
}

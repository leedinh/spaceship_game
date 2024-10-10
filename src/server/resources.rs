use bevy::{prelude::Resource, utils::HashMap};
use bevy_renet::renet::ClientId;
use spaceship_game::PlayerAttributes;

#[derive(Resource, Clone)]
pub struct PlayerLobby(pub HashMap<ClientId, PlayerAttributes>);

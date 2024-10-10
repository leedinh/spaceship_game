use bevy::{
    prelude::{Entity, Resource},
    utils::hashbrown::HashMap,
};
use bevy_renet::renet::ClientId;

#[derive(Resource)]
pub struct MyClientId(pub ClientId);

#[derive(Resource)]
pub struct PlayerEntities(pub HashMap<ClientId, Entity>);

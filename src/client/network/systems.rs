use bevy::log::info;
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use spaceship_game::PlayerAttributes;

use crate::network::PlayerEntity;

use super::{LobbySyncEvent, MyClientId, MyPlayer, PlayerDespawnEvent, PlayerSpawnEvent};

pub fn setup_system(mut commands: Commands) {
    commands.spawn(MyPlayer);
}

pub fn send_message_system(mut client: ResMut<RenetClient>, query: Query<(&MyPlayer, &Transform)>) {
    let (_, transform) = query.single();
    let player_sync = PlayerAttributes {
        position: transform.translation.into(),
    };
    let message = bincode::serialize(&player_sync).unwrap();
    client.send_message(DefaultChannel::Unreliable, message);
}

pub fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut spawn_events: EventWriter<PlayerSpawnEvent>,
    mut despawn_events: EventWriter<PlayerDespawnEvent>,
    mut lobby_sync_events: EventWriter<LobbySyncEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            spaceship_game::ServerMessage::PlayerJoin(client_id) => {
                info!("Client connected: {}", client_id);
                spawn_events.send(PlayerSpawnEvent(client_id));
            }
            spaceship_game::ServerMessage::PlayerLeave(client_id) => {
                info!("Client disconnected: {}", client_id);
                despawn_events.send(PlayerDespawnEvent(client_id));
            }
            _ => {
                info!("Unhandled message: {:?}", server_message);
            }
        }
    }
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let message = bincode::deserialize(&message).unwrap();

        match message {
            spaceship_game::ServerMessage::LobbySync(map) => {
                lobby_sync_events.send(LobbySyncEvent(map));
            }
            _ => {
                info!("Unhandled message: {:?}", message);
            }
        }
    }
}

pub fn handle_player_spawn_event_system(
    mut commands: Commands,
    mut spawn_events: EventReader<PlayerSpawnEvent>,
) {
    for event in spawn_events.read() {
        info!("Handling player spawn event: {:?}", event.0);
        let client_id = event.0;

        commands.spawn(PlayerEntity(client_id));
    }
}

pub fn handle_lobby_sync_event_system(
    mut spawn_events: EventWriter<PlayerSpawnEvent>,
    mut sync_events: EventReader<LobbySyncEvent>,
    mut query: Query<(&PlayerEntity, &mut Transform)>,
    my_clinet_id: Res<MyClientId>,
) {
    let event_option = sync_events.read().last();
    if event_option.is_none() {
        return;
    }
    let event = event_option.unwrap();

    for (client_id, player_sync) in event.0.iter() {
        if *client_id == my_clinet_id.0 {
            continue;
        }

        let mut found = false;
        for (player_entity, mut transform) in query.iter_mut() {
            if *client_id == player_entity.0 {
                let new_position = player_sync.position;
                transform.translation = new_position.into();
                found = true;
            }
        }

        if !found {
            info!("Spawning player {}: {:?}", client_id, player_sync.position);
            spawn_events.send(PlayerSpawnEvent(*client_id));
        }
    }
}

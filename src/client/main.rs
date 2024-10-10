mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod network;
mod schedule;
mod spaceship;
mod state;

use std::{
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    time::SystemTime,
};

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::{prelude::*, utils::HashMap};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ClientId, ConnectionConfig, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use network::{
    handle_lobby_sync_event_system, handle_player_spawn_event_system, receive_message_system,
    send_message_system, setup_system, LobbySyncEvent, MyClientId, PlayerDespawnEvent,
    PlayerEntities, PlayerMoveEvent, PlayerSpawnEvent,
};
use spaceship::SpaceshipPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_plugins(DefaultPlugins);

    let client = RenetClient::new(ConnectionConfig::default());
    app.insert_resource(client);

    let client_id = rand::random::<u64>();
    app.insert_resource(MyClientId(ClientId::from_raw(client_id)));
    app.insert_resource(PlayerEntities(HashMap::new()));

    let authentication = ClientAuthentication::Unsecure {
        server_addr: SocketAddr::V4(SocketAddrV4::new([127, 0, 0, 1].into(), 5000)),
        client_id,
        user_data: None,
        protocol_id: 0,
    };

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    app.insert_resource(transport);

    // game events
    app.add_event::<PlayerSpawnEvent>();
    app.add_event::<PlayerDespawnEvent>();
    app.add_event::<PlayerMoveEvent>();
    app.add_event::<LobbySyncEvent>();

    app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 2000.,
        })
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin);
    //     .add_plugins(MovementPlugin)
    //     // .add_plugins(DebugPlugin)
    //     .add_plugins((SpaceshipPlugin, AsteroidPlugin))
    //     .add_plugins(collision_detection::CollisionDetectionPlugin)
    //     .add_plugins(despawn::DespawnPlugin)
    //     .add_plugins(CameraPlugin)
    //     .add_plugins(schedule::SchedulePlugin)
    //     .add_plugins(state::GameStatePlugin)
    //     .run();

    // game systems
    // app.add_systems(Startup, setup_system);
    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_player_spawn_event_system);
    // app.add_systems(Update, update_player_movement_system);
    app.add_systems(Update, handle_lobby_sync_event_system);

    info!("Client {} started", client_id);
    app.run();
}

mod resources;
mod systems;

use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, RenetServer,
    },
    RenetServerPlugin,
};
use systems::setup_system;
// use tracing::level_filters::LevelFilter;
// use tracing_subscriber::{fmt, prelude::*};

const SERVER_ADDR: &str = "127.0.0.1:5000";

fn main() {
    let mut app = App::new();
    // base plugins
    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin::default());
    app.add_plugins(RenetServerPlugin);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    app.insert_resource(transport);

    app.insert_resource(resources::PlayerLobby(Default::default()))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                systems::send_message_system,
                systems::receive_message_system,
                systems::handle_events_system,
            ),
        )
        .run();
}

//! This is a very personal program that manages my JACK connections, making up for shortcomings of QJackCtl
#![warn(missing_docs)]
use jack as m;

use m::{Client, PortId, ClientStatus};

fn main() {
    let (mate, mate_ahoy) = m::Client::new(
        "jacks_mate",
        m::ClientOptions::USE_EXACT_NAME | m::ClientOptions::NO_START_SERVER,
    )
    .expect("Critical failure while connecting to JACK");

    let ok_to_go = !matches!(mate_ahoy, ClientStatus::NAME_NOT_UNIQUE | ClientStatus::INVALID_OPTION );
    // Name uniqueness is checked so there's no multiple instances of this program operating on the graph
    // Invalid option passed suggests JACK running but being way ahead or behind in version than what we support

    if !ok_to_go {
        panic!("Connected to JACK but unable to proceed!");
    }

    let ports = mate.ports(None, None, m::PortFlags::empty());

    for port in ports {
        println!("Port found: {}", port);

        match mate.port_by_name(port.as_str()) {
            Some(port_struct) => {
                println!("{:?}", port_struct);
            }
            None => println!("Unfortunately unable to get port_struct."),
        }
    }

    let connectome_police = ConnectionNotificationHandler {};

    let _the_operation = mate
        .activate_async(connectome_police, ())
        .expect("Asynchronous operation start failed critically!");

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}

struct ConnectionNotificationHandler {}

impl m::NotificationHandler for ConnectionNotificationHandler {
    fn ports_connected(
        &mut self,
        client: &Client,
        port_id_a: PortId,
        port_id_b: PortId,
        are_connected: bool,
    ) {
        let name_of_port = |port_id: PortId, default_name: String| -> String {
            client.port_by_id(port_id).map_or_else(
                || default_name.to_owned(),
                |port| port.name().unwrap_or_else(|_| default_name.to_owned()),
            )
        };

        let (name_of_a, name_of_b) = (
            name_of_port(port_id_a, "unnamed source port".to_string()),
            name_of_port(port_id_b, "unnamed sink port".to_string()),
        );

        println!(
            "Ports {} and {} are observed {}.",
            name_of_a,
            name_of_b,
            match are_connected {
                true => "connected",
                false => "disconnected",
            }
        );
    }
    fn port_registration(&mut self, client: &Client, port_id: PortId, is_registered: bool) {
        let name_of_port = |port_id: PortId, default_name: String| -> String {
            client.port_by_id(port_id).map_or_else(
                || default_name.to_owned(),
                |port| port.name().unwrap_or_else(|_| default_name.to_owned()),
            )
        };

        println!(
            "Port {} has been observed {}.",
            name_of_port(port_id, "unknown port name".to_string()),
            match is_registered {
                true => "registered",
                false => "unregistered",
            }
        );
    }
}

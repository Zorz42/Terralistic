use std::any::Any;
use std::io::{Read, Write};
use std::net::TcpStream;
use bincode;
use shared::packet::{Packet, WelcomeCompletePacket};
use uflow::client::Client;
use events::EventManager;

/**
This event is called, when the client has received a welcome packet.
 */
pub struct WelcomePacketEvent {
    pub packet: Packet,
}

/**
This handles all the networking for the client.
Client connects to a server and sends and receives packets.
 */
pub struct ClientNetworking {
    server_port: u16,
    server_address: String,
    net_client: Option<Client>,
}

impl ClientNetworking {
    pub fn new(server_port: u16, server_address: String) -> Self {
        Self {
            server_port,
            server_address,
            net_client: None,
        }
    }

    pub fn init(&mut self, events: &mut EventManager) {
        // connect to the server
        self.net_client = Some(Client::connect(format!("{}:{}", self.server_address, self.server_port), Default::default()).unwrap());

        'welcome_loop: loop {
            for event in self.net_client.as_mut().unwrap().step() {
                match event {
                    uflow::client::Event::Connect => {}
                    uflow::client::Event::Disconnect => {
                        panic!("disconnected from server");
                    }
                    uflow::client::Event::Error(err) => {
                        panic!("server connection error: {:?}", err);
                    }
                    uflow::client::Event::Receive(packet_data) => {
                        let packet = bincode::deserialize::<Packet>(&packet_data).unwrap();
                        if let Some(_) = packet.deserialize::<WelcomeCompletePacket>() {
                            break 'welcome_loop;
                        }

                        println!("got packet with size {}", packet_data.len());

                        // send welcome packet event
                        events.push_event(Box::new(
                            WelcomePacketEvent {
                                packet,
                            }
                        ));
                    }
                }
            }
            self.net_client.as_mut().unwrap().flush();
        }
    }

    pub fn on_event(&mut self, event: Box<dyn Any>) {
        // check if event is a packet
        if let Some(_packet) = event.downcast_ref::<Packet>() {

        }
    }

    pub fn update(&mut self, events: &mut EventManager) {
        // flush the stream
        self.net_client.as_mut().unwrap().flush();
        // get all packets from the server one by one
        for event in self.net_client.as_mut().unwrap().step() {
            match event {
                uflow::client::Event::Connect => {
                    panic!("connected to server at incorrect time");
                }
                uflow::client::Event::Disconnect => {
                    panic!("disconnected from server");
                }
                uflow::client::Event::Error(err) => {
                    panic!("server connection error: {:?}", err);
                }
                uflow::client::Event::Receive(packet_data) => {
                    let packet = bincode::deserialize::<Packet>(&packet_data).unwrap();

                    // send welcome packet event
                    events.push_event(Box::new(packet));
                }
            }
        }
    }

    pub fn stop(&mut self) {
        // disconnect the socket
        self.net_client.as_mut().unwrap().disconnect();
    }
}
use bincode;
use enet::{Address, BandwidthLimit, ChannelLimit, Event, Host};
use events::EventManager;
use shared::enet_global::ENET_GLOBAL;
use shared::packet::{Packet, WelcomeCompletePacket};

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
    net_client: Option<Host<()>>,
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
        self.net_client = Some(
            ENET_GLOBAL
                .create_host::<()>(
                    None,
                    10,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .unwrap(),
        );

        self.net_client
            .as_mut()
            .unwrap()
            .connect(
                &Address::new(self.server_address.parse().unwrap(), self.server_port),
                10,
                0,
            )
            .unwrap();

        loop {
            if let Some(event) = self.net_client.as_mut().unwrap().service(0).unwrap() {
                match event {
                    Event::Connect(ref _peer) => {
                        break;
                    }
                    Event::Disconnect { .. } => {
                        panic!("disconnected from server");
                    }
                    Event::Receive { .. } => {
                        panic!("unexpected receive");
                    }
                };
            }
        }

        'welcome_loop: loop {
            if let Some(event) = self.net_client.as_mut().unwrap().service(0).unwrap() {
                match event {
                    Event::Connect(_) => {
                        panic!("unexpected connect");
                    }
                    Event::Disconnect { .. } => {
                        panic!("disconnected from server");
                    }
                    Event::Receive { ref packet, .. } => {
                        let packet = bincode::deserialize::<Packet>(&packet.data()).unwrap();

                        if let Some(_) = packet.deserialize::<WelcomeCompletePacket>() {
                            break 'welcome_loop;
                        }

                        // send welcome packet event
                        events.push_event(events::Event::new(Box::new(WelcomePacketEvent {
                            packet,
                        })));
                    }
                };
            }
        }
    }

    pub fn update(&mut self, events: &mut EventManager) {
        self.net_client.as_mut().unwrap().flush();

        while let Some(event) = self.net_client.as_mut().unwrap().service(0).unwrap() {
            match event {
                Event::Connect(_) => {
                    panic!("connected to server at incorrect time");
                }
                Event::Disconnect { .. } => {
                    panic!("disconnected from server");
                }
                Event::Receive { ref packet, .. } => {
                    let packet = bincode::deserialize::<Packet>(&packet.data()).unwrap();

                    // send welcome packet event
                    events.push_event(events::Event::new(Box::new(packet)));
                }
            };
        }
    }

    pub fn stop(&mut self) {
        // disconnect the socket
        for ref mut server in self.net_client.as_mut().unwrap().peers() {
            server.disconnect(0);
        }
    }
}

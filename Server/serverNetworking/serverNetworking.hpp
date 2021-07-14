//
//  serverNetworking.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#define MAX_PLAYERS 100

#include <string>
#include <thread>
#include <vector>
#include <SFML/Network.hpp>

#include "events.hpp"
#include "packetType.hpp"

class connection {
    sf::Socket socket;
public:
    std::string ip;
    bool disconnected = false, registered = false;
    
    int getSocket();
    void setSocket(int socket);
    bool isConnected();
    
    sf::Packet getPacket();
    void sendPacket(const sf::Packet& packet_);
};

class serverNetworkingManager {
    bool listener_running = false;
    int server_fd;
    unsigned short port;
    std::thread listener_thread;
    void onPacket(sf::Packet& packet, connection& conn);
    void listenerLoop();
public:
    serverNetworkingManager(unsigned short port) : port(port) {}
    
    bool accept_only_itself = false; // only accept 0.0.0.0 Used in singleplayer to basically have a private server
    connection connections[MAX_PLAYERS];
    
    void startListening();
    void stopListening();
    void sendToEveryone(const sf::Packet& packet, connection* exclusion=nullptr);
    
    inline unsigned short getPort() { return port; }
};

#endif /* serverNetworking_hpp */

#ifndef clientNetworking_hpp
#define clientNetworking_hpp

#include <SFML/Network.hpp>
#include "events.hpp"
#include "packetType.hpp"

class NetworkingManager {
    sf::TcpSocket socket;
    sf::Packet master_packet;
public:
    bool establishConnection(const std::string& ip, unsigned short port);
    void closeConnection();
    void checkForPackets();
    void sendPacket(sf::Packet& packet);
    void disableBlocking();
    sf::Packet getPacket();
    std::vector<char> getData(unsigned int size);
    void flushPackets();
};

class ClientPacketEvent : public Event<ClientPacketEvent> {
public:
    ClientPacketEvent(sf::Packet& packet, PacketType packet_type) : packet(packet), packet_type(packet_type) {}
    sf::Packet& packet;
    PacketType packet_type;
};

#endif

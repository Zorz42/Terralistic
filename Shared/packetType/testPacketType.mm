#include "testing.hpp"
#include "networking.hpp"
#include "packetType.hpp"
#include <string>

TEST_CLASS(TestPacketType)

TEST_CASE(testClientPacketTypeSaves) {
    Packet packet;
    
    packet << ClientPacketType::PING;
    ClientPacketType received;
    packet >> received;
    ASSERT(received == ClientPacketType::PING);
    
    for(int type = (int)ClientPacketType::_START + 1; type < (int)ClientPacketType::_END - 1; type++) {
        packet << (ClientPacketType)type;
        ClientPacketType received_type;
        packet >> received_type;
        ASSERT(received_type == (ClientPacketType)type);
    }
}

TEST_CASE(testServerPacketTypeSaves) {
    Packet packet;
    
    packet << ServerPacketType::PING;
    ServerPacketType received;
    packet >> received;
    ASSERT(received == ServerPacketType::PING);
    
    for(int type = (int)ServerPacketType::_START + 1; type < (int)ServerPacketType::_END - 1; type++) {
        packet << (ServerPacketType)type;
        ServerPacketType received_type;
        packet >> received_type;
        ASSERT(received_type == (ServerPacketType)type);
    }
}

END_TEST_CLASS(TestCompress)

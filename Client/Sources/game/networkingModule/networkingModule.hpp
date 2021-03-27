//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#define PACKET_LISTENER(packetType) static networking::registerPacketListener JOIN(listener_func, __LINE__) (packetType, [](packets::packet& packet) {
#define PACKET_LISTENER_END });

namespace networking {

struct networkingManager {
    bool startListening(const std::string& ip);
    void stopListening();

    void sendPacket(packets::packet packet_);
private:
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networking::networkingManager* manager);
    std::vector<packets::packet> packet_queue;
};

typedef void(*listenerFunction)(packets::packet&);

struct registerPacketListener {
    registerPacketListener(packets::packetType type, listenerFunction function);
};

}

#endif /* networkingModule_hpp */

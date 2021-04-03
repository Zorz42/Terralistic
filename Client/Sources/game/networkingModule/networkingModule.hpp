//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#include <vector>
#include <set>

namespace networking {

struct networkingManager;

struct packetListener {
    packetListener(networkingManager* manager);
    virtual ~packetListener() = default;
    std::set<packets::packetType> listening_to;
    virtual void onPacket(packets::packet packet) = 0;
};

struct networkingManager {
    bool startListening(const std::string& ip);
    void stopListening();

    void sendPacket(packets::packet packet_);
    void registerListener(packetListener* listener);
private:
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networking::networkingManager* manager);
    std::vector<packetListener*> listeners;
};

}

#endif /* networkingModule_hpp */

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

struct packetListener {
    virtual ~packetListener() {}
    std::set<packets::packetType> listening_to;
    virtual void onPacket(packets::packet packet) = 0;
};

struct networkingManager {
    bool startListening(const std::string& ip);
    void stopListening();

    void sendPacket(packets::packet packet_);
    std::vector<packetListener*> listeners;
private:
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networking::networkingManager* manager);
};

}

#endif /* networkingModule_hpp */

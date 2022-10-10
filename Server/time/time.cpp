#include "time.hpp"

void ServerTime::init() {
    networking->new_connection_event.addListener(this);
}

void ServerTime::stop() {
    networking->new_connection_event.removeListener(this);
}

void ServerTime::setTime(int time) {
    timer.set(time);
    Packet time_packet;
    time_packet << ServerPacketType::TIME << time;
    networking->sendToEveryone(time_packet);
}

void ServerTime::onEvent(ServerNewConnectionEvent& event) {
    Packet time_packet;
    time_packet << ServerPacketType::TIME << timer.getTimeElapsed();
    event.connection->send(time_packet);
}

#include "serverBlocks.hpp"

void ServerBlocks::onEvent(ServerConnectionWelcomeEvent& event) {
    sf::Packet packet;
    packet << WelcomePacketType::BLOCKS;
    event.connection->sendDirectly(packet);
    
    event.connection->send(toSerial());
}

void ServerBlocks::init() {    
    networking->connection_welcome_event.addListener(this);
    block_change_event.addListener(this);
    block_started_breaking_event.addListener(this);
    block_stopped_breaking_event.addListener(this);
    world_saver->world_load_event.addListener(this);
    world_saver->world_save_event.addListener(this);
    block_update_event.addListener(this);
}

void ServerBlocks::update(float frame_length) {
    updateBreakingBlocks(frame_length);
}

void ServerBlocks::stop() {
    networking->connection_welcome_event.removeListener(this);
    block_change_event.removeListener(this);
    block_started_breaking_event.removeListener(this);
    block_stopped_breaking_event.removeListener(this);
    world_saver->world_load_event.removeListener(this);
    world_saver->world_save_event.removeListener(this);
    block_update_event.removeListener(this);
}

void ServerBlocks::onEvent(BlockChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK << event.x << event.y << (unsigned char)getBlockType(event.x, event.y)->id << (unsigned char)getBlockXFromMain(event.x, event.y) << (unsigned char)getBlockYFromMain(event.x, event.y);
    networking->sendToEveryone(packet);
    
    int neighbours[5][2] = {{event.x, event.y}, {event.x - 1, event.y}, {event.x, event.y - 1}, {event.x + 1, event.y}, {event.x, event.y + 1}};
    for(int i = 0; i < 5; i++)
        if(neighbours[i][0] >= 0 && neighbours[i][0] < getWidth() && neighbours[i][1] >= 0 && neighbours[i][1] < getHeight())
            updateBlock(neighbours[i][0], neighbours[i][1]);
}

void ServerBlocks::updateBlock(int x, int y) {
    BlockUpdateEvent update_event(x, y);
    block_update_event.call(update_event);
}

void ServerBlocks::onEvent(BlockUpdateEvent& event) {
    if(getBlockType(event.x, event.y)->width != 0) {
        int x_offset = getBlockXFromMain(event.x, event.y), y_offset = getBlockYFromMain(event.x, event.y);
        
        if((x_offset > 0 && y_offset == 0 && event.x != 0 && getBlockType(event.x - 1, event.y) != getBlockType(event.x, event.y)) ||
           (y_offset > 0 && event.y != 0 && getBlockType(event.x, event.y - 1) != getBlockType(event.x, event.y))) {
            setBlockType(event.x, event.y, &air);
        } else {
            if(event.x < getWidth() - 1 && x_offset + 1 < getBlockType(event.x, event.y)->width)
                setBlockType(event.x + 1, event.y, getBlockType(event.x, event.y), x_offset + 1, y_offset);
            if(event.y < getHeight() - 1 && y_offset + 1 < getBlockType(event.x, event.y)->height)
                setBlockType(event.x, event.y + 1, getBlockType(event.x, event.y), x_offset, y_offset + 1);
        }
        
        
    }
}

void ServerBlocks::onEvent(BlockStartedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK_STARTED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockStoppedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK_STOPPED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(WorldSaveEvent &event) {
    world_saver->setSectionData("blocks", toSerial());
}

void ServerBlocks::onEvent(WorldLoadEvent &event) {
    fromSerial(world_saver->getSectionData("blocks"));
}

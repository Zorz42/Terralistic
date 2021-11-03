#include <filesystem>
#include <fstream>
#include <utility>
#include <cassert>
#include "serverPlayers.hpp"
#include "blocks.hpp"
#include "print.hpp"

static bool isBlockTree(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == BlockTypeOld::WOOD || blocks->getBlockType(x, y) == BlockTypeOld::LEAVES);
}

static bool isBlockWood(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == BlockTypeOld::WOOD;
}

static bool isBlockLeaves(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == BlockTypeOld::LEAVES;
}

static void stoneUpdate(Blocks* blocks, unsigned short x, unsigned short y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockInfo(x, y + 1).transparent)
        blocks->breakBlock(x, y);
}

static void treeUpdate(Blocks* blocks, unsigned short x, unsigned short y) {
    if(
       (!isBlockTree(blocks, x, y + 1) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (!isBlockTree(blocks, x, y + 1) && isBlockLeaves(blocks, x - 1, y) && isBlockLeaves(blocks, x + 1, y) && isBlockLeaves(blocks, x, y - 1))
       )
        blocks->breakBlock(x, y);
}

void ServerPlayers::init() {
    blocks->block_change_event.addListener(this);
    networking->new_connection_event.addListener(this);
    networking->connection_welcome_event.addListener(this);
    packet_event.addListener(this);
    networking->disconnect_event.addListener(this);
    
    custom_block_events[(int)BlockTypeOld::WOOD].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockTypeOld::LEAVES].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockTypeOld::GRASS_BLOCK].onLeftClick = [](Blocks* blocks_, unsigned short x, unsigned short y, ServerPlayer* player) {
        blocks_->setBlockType(x, y, BlockTypeOld::DIRT);
    };

    custom_block_events[(int)BlockTypeOld::AIR].onRightClick = [](Blocks* blocks_, unsigned short x, unsigned short y, ServerPlayer* player) {
        BlockTypeOld type = ::getItemInfoOld(player->inventory.getSelectedSlot().type).places;
        if(type != BlockTypeOld::AIR && player->inventory.decreaseStack(player->inventory.selected_slot, 1)) {
            blocks_->setBlockType(x, y, type);
        }
    };

    custom_block_events[(int)BlockTypeOld::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockTypeOld::GRASS_BLOCK].onLeftClick;
    
    custom_block_events[(int)BlockTypeOld::STONE].onUpdate = &stoneUpdate;
}

void ServerPlayers::stop() {
    blocks->block_change_event.removeListener(this);
    networking->new_connection_event.removeListener(this);
    networking->connection_welcome_event.removeListener(this);
    packet_event.removeListener(this);
    networking->disconnect_event.removeListener(this);
    
    for(ServerPlayerData* i : all_players)
        delete i;
}

ServerPlayer* ServerPlayers::getPlayerByName(const std::string& name) {
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entity;
            if(player->name == name)
                return player;
        }
    return nullptr;
}

ServerPlayer* ServerPlayers::addPlayer(const std::string& name) {
    ServerPlayerData* player_data = getPlayerData(name);
    
    if(!player_data) {
        int spawn_x = blocks->getWidth() / 2 * BLOCK_WIDTH * 2;
        
        int spawn_y = 0;
        for(unsigned short y = 0; y < blocks->getHeight(); y++) {
            if(!blocks->getBlockInfo(blocks->getWidth() / 2, y).transparent || !blocks->getBlockInfo(blocks->getWidth() / 2 + 1, y).transparent)
                break;
            spawn_y += BLOCK_WIDTH * 2;
        }
        spawn_y -= PLAYER_HEIGHT * 2;
        
        player_data = new ServerPlayerData();
        player_data->name = name;
        player_data->x = spawn_x;
        player_data->y = spawn_y;
        all_players.emplace_back(player_data);
    }
    
    ServerPlayer* player = new ServerPlayer(*player_data);
    entities->registerEntity(player);
    return player;
}

void ServerPlayers::savePlayer(ServerPlayer* player) {
    ServerPlayerData* player_data = getPlayerData(player->name);
    
    player->destruct();
    
    player_data->name = player->name;
    player_data->x = player->getX();
    player_data->y = player->getY();
    player_data->inventory = player->inventory;
}

ServerPlayerData* ServerPlayers::getPlayerData(const std::string& name) {
    for(ServerPlayerData* data : all_players)
        if(data->name == name)
            return data;
    return nullptr;
}

void ServerPlayers::leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y) {
    while(custom_block_events[(int)blocks->getBlockType(x, y)].onLeftClick)
        custom_block_events[(int)blocks->getBlockType(x, y)].onLeftClick(blocks, x, y, player);
    
    if(blocks->getBlockInfo(x, y).break_time != UNBREAKABLE)
        blocks->startBreakingBlock(x, y);
}

void ServerPlayers::rightClickEvent(ServerPlayer* player, unsigned short x, unsigned short y) {
    if(custom_block_events[(int)blocks->getBlockType(x, y)].onRightClick)
        custom_block_events[(int)blocks->getBlockType(x, y)].onRightClick(blocks, x, y, player);
}

char* ServerPlayers::addPlayerFromSerial(char* iter) {
    all_players.emplace_back(new ServerPlayerData(iter));
    return iter;
}

ServerPlayerData::ServerPlayerData(char*& iter) {
    iter = inventory.loadFromSerial(iter);
    
    x = 0;
    for(int i = 0; i < sizeof(int); i++)
        x += (int)(unsigned char)*iter++ << i * 8;
    y = 0;
    for(int i = 0; i < sizeof(int); i++)
        y += (int)(unsigned char)*iter++ << i * 8;
    
    while(*iter)
        name.push_back(*iter++);
    iter++;
}

void ServerPlayerData::serialize(std::vector<char>& serial) const {
    inventory.serialize(serial);
    
    for(int i = 0; i < sizeof(int); i++)
        serial.push_back(x >> i * 8);
    
    for(int i = 0; i < sizeof(int); i++)
        serial.push_back(y >> i * 8);
    
    serial.insert(serial.end(), name.begin(), name.end());
    serial.insert(serial.end(), 0);
}

void ServerPlayers::onEvent(BlockChangeEvent& event) {
    int neighbors[5][2] = {{event.x, event.y}, {-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    
    if(event.x != 0) {
        neighbors[1][0] = event.x - 1;
        neighbors[1][1] = event.y;
    }
    if(event.x != blocks->getWidth() - 1) {
        neighbors[2][0] = event.x + 1;
        neighbors[2][1] = event.y;
    }
    if(event.y != 0) {
        neighbors[3][0] = event.x;
        neighbors[3][1] = event.y - 1;
    }
    if(event.y != blocks->getHeight() - 1) {
        neighbors[4][0] = event.x;
        neighbors[4][1] = event.y + 1;
    }
    
    for(auto neighbor : neighbors)
        if(neighbor[0] != -1 && custom_block_events[(int)blocks->getBlockType(neighbor[0], neighbor[1])].onUpdate)
            custom_block_events[(int)blocks->getBlockType(neighbor[0], neighbor[1])].onUpdate(blocks, neighbor[0], neighbor[1]);
}

void ServerPlayers::onEvent(ServerNewConnectionEvent& event) {
    ServerPlayer* player = nullptr;
    
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* curr_player = (ServerPlayer*)entity;
            if(curr_player->getConnection() == event.connection) {
                player = curr_player;
                break;
            } else {
                sf::Packet join_packet;
                join_packet << ServerPacketType::PLAYER_JOIN << curr_player->getX() << curr_player->getY() << curr_player->id << curr_player->name << (unsigned char)curr_player->moving_type;
                event.connection->send(join_packet);
            }
        }
    
    assert(player);
    sf::Packet join_packet;
    join_packet << ServerPacketType::PLAYER_JOIN << player->getX() << player->getY() << player->id << player->name << (unsigned char)player->moving_type;
    networking->sendToEveryone(join_packet);
}

void ServerPlayers::onEvent(ServerConnectionWelcomeEvent& event) {
    std::string player_name;
    event.client_welcome_packet >> player_name;
    
    ServerPlayer* already_joined_player = getPlayerByName(player_name);
    if(already_joined_player)
        networking->kickConnection(already_joined_player->getConnection(), "You logged in from another location!");
    
    ServerPlayer* player = addPlayer(player_name);
    player->setConnection(event.connection);
    
    sf::Packet packet;
    packet << WelcomePacketType::INVENTORY;
    event.connection->send(packet);
    
    std::vector<char> data;
    player->inventory.serialize(data);
    event.connection->send(data);
}

void ServerPlayer::setConnection(Connection* connection_) {
    assert(connection == nullptr);
    connection = connection_;
}

Connection* ServerPlayer::getConnection() {
    return connection;
}

void ServerPlayers::update(float frame_length) {
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entity;
            while(player->getConnection()->hasPacketInBuffer()) {
                auto result = player->getConnection()->getPacket();
                
                ServerPacketEvent event(result.first, result.second, player);
                packet_event.call(event);
            }
        }

    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::ITEM) {
            Item* item = (Item*)entity;
            for(Entity* entity2 : entities->getEntities())
                if(entity2->type == EntityType::PLAYER) {
                    ServerPlayer* player = (ServerPlayer*)entity2;
                    if(abs(item->getX() + BLOCK_WIDTH - player->getX() - 14) < 50 && abs(item->getY() + BLOCK_WIDTH - player->getY() - 25) < 50 &&
                       player->inventory.addItem(item->getType(), 1) != -1
                       ) {
                        entities->removeEntity(item);
                    }
                }
        }
}

void ServerPlayers::onEvent(ServerDisconnectEvent& event) {
    ServerPlayer* player = nullptr;
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* player_ = (ServerPlayer*)entity;
            if(player_->getConnection() == event.connection) {
                player = player_;
                break;
            }
        }
    assert(player);
    savePlayer(player);
    entities->removeEntity(player);
}

void ServerPlayers::onEvent(ServerPacketEvent& event) {
    switch(event.packet_type) {
        case ClientPacketType::STARTED_BREAKING: {
            int breaking_x, breaking_y;
            event.packet >> breaking_x >> breaking_y;
            
            if(event.player->breaking)
                blocks->stopBreakingBlock(event.player->breaking_x, event.player->breaking_y);
            
            event.player->breaking_x = breaking_x;
            event.player->breaking_y = breaking_y;
            event.player->breaking = true;
            leftClickEvent(event.player, breaking_x, breaking_y);
            break;
        }

        case ClientPacketType::STOPPED_BREAKING: {
            event.player->breaking = false;
            blocks->stopBreakingBlock(event.player->breaking_x, event.player->breaking_y);
            break;
        }

        case ClientPacketType::RIGHT_CLICK: {
            int x, y;
            event.packet >> x >> y;
            rightClickEvent(event.player, x, y);
            break;
        }

        case ClientPacketType::PLAYER_VELOCITY: {
            float velocity_x, velocity_y;
            event.packet >> velocity_x >> velocity_y;
            entities->setVelocityX(event.player, velocity_x);
            entities->setVelocityY(event.player, velocity_y);
            break;
        }

        case ClientPacketType::INVENTORY_SWAP: {
            unsigned char pos;
            event.packet >> pos;
            event.player->inventory.swapWithMouseItem(pos);
            break;
        }

        case ClientPacketType::HOTBAR_SELECTION: {
            event.packet >> event.player->inventory.selected_slot;
            break;
        }
            
        case ClientPacketType::CRAFT: {
            unsigned char craft_index;
            event.packet >> craft_index;
            const RecipeOld* recipe_crafted = event.player->inventory.getAvailableRecipes()[(int)craft_index];
            event.player->inventory.addItem(recipe_crafted->result_type, recipe_crafted->result_stack);
            
            for(auto ingredient : recipe_crafted->ingredients)
                event.player->inventory.removeItem(ingredient.first, ingredient.second);
        }
            
        case ClientPacketType::PLAYER_MOVING_TYPE: {
            unsigned char moving_type;
            event.packet >> moving_type;
            event.player->moving_type = (MovingType)moving_type;
            sf::Packet moving_packet;
            moving_packet << ServerPacketType::PLAYER_MOVING_TYPE << moving_type << event.player->id;
            networking->sendToEveryone(moving_packet);
        }

        case ClientPacketType::PLAYER_JUMPED: {
            sf::Packet jumped_packet;
            jumped_packet << ServerPacketType::PLAYER_JUMPED << event.player->id;
            networking->sendToEveryone(jumped_packet);
        }
        default:;
    }
}

void ServerPlayer::onEvent(InventoryItemChangeEvent& event) {
    ItemStack item = inventory.getItem(event.item_pos);
    sf::Packet packet;
    packet << ServerPacketType::INVENTORY << item.stack << (unsigned char)item.type << (short)event.item_pos;
    connection->send(packet);
}

void ServerPlayer::destruct() {
    inventory.item_change_event.removeListener(this);
}

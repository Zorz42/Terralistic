#include "serverPlayers.hpp"
#include "content.hpp"
#include <cstring>

void AirBehaviour::onRightClick(Blocks* blocks, int x, int y, ServerPlayer* player) {
    BlockType* type = player->inventory.getSelectedSlot().type->places;
    if(type != &blocks->air && player->inventory.decreaseStack(player->inventory.selected_slot, 1)) {
        blocks->setBlockType(x, y, type);
    }
}

void ServerPlayers::init() {
    blocks_behaviour = new BlockBehaviour*[blocks->getNumBlockTypes()];
    for(int i = 0; i < blocks->getNumBlockTypes(); i++)
        blocks_behaviour[i] = &default_behaviour;
    
    getBlockBehaviour(&blocks->air) = &air_behaviour;
    
    blocks->block_change_event.addListener(this);
    networking->new_connection_event.addListener(this);
    networking->connection_welcome_event.addListener(this);
    packet_event.addListener(this);
    networking->disconnect_event.addListener(this);
}

void ServerPlayers::stop() {
    blocks->block_change_event.removeListener(this);
    networking->new_connection_event.removeListener(this);
    networking->connection_welcome_event.removeListener(this);
    packet_event.removeListener(this);
    networking->disconnect_event.removeListener(this);
    
    for(int i = 0; i < all_players.size(); i++)
        delete all_players[i];
    
    delete[] blocks_behaviour;
}

ServerPlayer* ServerPlayers::getPlayerByName(const std::string& name) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entities->getEntities()[i];
            if(player->name == name)
                return player;
        }
    return nullptr;
}

ServerPlayer* ServerPlayers::addPlayer(const std::string& name) {
    ServerPlayerData* player_data = getPlayerData(name);
    
    if(!player_data) {
        int spawn_x = blocks->getWidth() / 2;
        int spawn_y = 0;
        while(blocks->getBlockType(spawn_x, spawn_y)->ghost && blocks->getBlockType(spawn_x + 1, spawn_y)->ghost)
            spawn_y++;

        player_data = new ServerPlayerData(items, recipes);
        player_data->name = name;
        player_data->x = spawn_x * BLOCK_WIDTH * 2;
        player_data->y = spawn_y * BLOCK_WIDTH * 2 - PLAYER_HEIGHT * 2;
        player_data->health = 50;
        all_players.emplace_back(player_data);
    }
    
    ServerPlayer* player = new ServerPlayer(*player_data);
    entities->registerEntity(player);
    return player;
}

void ServerPlayers::savePlayer(ServerPlayer* player) {
    ServerPlayerData* player_data = getPlayerData(player->name);
    
    player_data->name = player->name;
    player_data->x = player->getX();
    player_data->y = player->getY();
    player_data->health = player->getHealth();
    player_data->inventory = player->inventory;
}

ServerPlayerData* ServerPlayers::getPlayerData(const std::string& name) {
    for(int i = 0; i < all_players.size(); i++)
        if(all_players[i]->name == name)
            return all_players[i];
    return nullptr;
}

void ServerPlayers::leftClickEvent(ServerPlayer* player, int x, int y) {
    while(true) {
        BlockType* type = blocks->getBlockType(x, y);
        getBlockBehaviour(blocks->getBlockType(x, y))->onLeftClick(blocks, x, y, player);
        if(blocks->getBlockType(x, y) == type)
            break;
    }
}

void ServerPlayers::rightClickEvent(ServerPlayer* player, int x, int y) {
    getBlockBehaviour(blocks->getBlockType(x, y))->onRightClick(blocks, x, y, player);
}

const char* ServerPlayers::addPlayerFromSerial(const char* iter) {
    all_players.emplace_back(new ServerPlayerData(items, recipes, iter));
    return iter;
}

ServerPlayerData::ServerPlayerData(Items* items, Recipes* recipes, const char*& iter) : inventory(items, recipes) {
    iter = inventory.loadFromSerial(iter);
    
    memcpy(&x, iter, sizeof(int));
    iter += sizeof(int);
    
    memcpy(&y, iter, sizeof(int));
    iter += sizeof(int);
    
    while(*iter)
        name.push_back(*iter++);
    iter++;

    memcpy(&health, iter, sizeof(short));
    iter += sizeof(short);
}

void ServerPlayerData::serialize(std::vector<char>& serial) const {
    inventory.serialize(serial);
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    memcpy(&serial[serial.size() - 4], &x, sizeof(int));
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    memcpy(&serial[serial.size() - 4], &y, sizeof(int));
    
    serial.insert(serial.end(), name.begin(), name.end());
    serial.insert(serial.end(), 0);

    serial.insert(serial.end(), {0, 0});
    memcpy(&serial[serial.size() - 4], &health, sizeof(short));
}

void ServerPlayers::onEvent(BlockChangeEvent& event) {
    int neighbours[5][2] = {{event.x, event.y}, {-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    
    if(event.x != 0) {
        neighbours[1][0] = event.x - 1;
        neighbours[1][1] = event.y;
    }
    if(event.x != blocks->getWidth() - 1) {
        neighbours[2][0] = event.x + 1;
        neighbours[2][1] = event.y;
    }
    if(event.y != 0) {
        neighbours[3][0] = event.x;
        neighbours[3][1] = event.y - 1;
    }
    if(event.y != blocks->getHeight() - 1) {
        neighbours[4][0] = event.x;
        neighbours[4][1] = event.y + 1;
    }
    
    for(int i = 0; i < 5; i++)
        if(neighbours[i][0] != -1)
            getBlockBehaviour(blocks->getBlockType(neighbours[i][0], neighbours[i][1]))->onUpdate(blocks, neighbours[i][0], neighbours[i][1]);
}

void ServerPlayers::onEvent(ServerNewConnectionEvent& event) {
    ServerPlayer* player = nullptr;
    
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* curr_player = (ServerPlayer*)entities->getEntities()[i];
            if(curr_player->getConnection() == event.connection) {
                player = curr_player;
                break;
            } else {
                sf::Packet join_packet;
                join_packet << ServerPacketType::PLAYER_JOIN << curr_player->getX() << curr_player->getY() << curr_player->id << curr_player->name << (int)curr_player->moving_type;
                event.connection->send(join_packet);
            }
        }
    
    if(player == nullptr)
        throw Exception("Could not find the player.");
    
    sf::Packet join_packet;
    join_packet << ServerPacketType::PLAYER_JOIN << player->getX() << player->getY() << player->id << player->name << (int)player->moving_type;
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
    if(connection)
        throw Exception("Overwriting connection, which has already been set");
    connection = connection_;
}

Connection* ServerPlayer::getConnection() {
    return connection;
}

void ServerPlayers::update(float frame_length) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entities->getEntities()[i];
            if(player->getVelocityX())
                player->flipped = player->getVelocityX() < 0;
            
            while(player->getConnection()->hasPacketInBuffer()) {
                auto result = player->getConnection()->getPacket();
                
                ServerPacketEvent event(result.first, result.second, player);
                packet_event.call(event);
            }
        }
        
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::ITEM) {
            Item* item = (Item*)entities->getEntities()[i];
            for(int i2 = 0; i2 < entities->getEntities().size(); i2++)
                if(entities->getEntities()[i2]->type == EntityType::PLAYER) {
                    ServerPlayer* player = (ServerPlayer*)entities->getEntities()[i2];
                    int distance_x = abs(item->getX() + BLOCK_WIDTH - player->getX() - 14);
                    int distance_y = abs(item->getY() + BLOCK_WIDTH - player->getY() - 25);
                    
                    if(distance_x < 50 && distance_y < 50) {
                        entities->addVelocityX(item, (player->getX() - item->getX()) * frame_length / 200.f);
                        entities->addVelocityY(item, (player->getY() - item->getY()) * frame_length / 70.f);
                    }
                    
                    if(distance_x < 10 && distance_y < 10 && player->inventory.addItem(item->getType(), 1) != -1)
                        entities->removeEntity(item);
                }
        }
}

void ServerPlayers::onEvent(ServerDisconnectEvent& event) {
    ServerPlayer* player = nullptr;
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* player_ = (ServerPlayer*)entities->getEntities()[i];
            if(player_->getConnection() == event.connection) {
                player = player_;
                break;
            }
        }
    
    if(player == nullptr)
        throw Exception("Could not find the player.");
    
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
            int pos;
            event.packet >> pos;
            event.player->inventory.swapWithMouseItem(pos);
            break;
        }

        case ClientPacketType::HOTBAR_SELECTION: {
            event.packet >> event.player->inventory.selected_slot;
            break;
        }
            
        case ClientPacketType::CRAFT: {
            int craft_index;
            event.packet >> craft_index;
            const Recipe* recipe_crafted = event.player->inventory.getAvailableRecipes()[(int)craft_index];
            event.player->inventory.addItem(recipe_crafted->result.type, recipe_crafted->result.stack);
            
            for(auto ingredient : recipe_crafted->ingredients)
                event.player->inventory.removeItem(ingredient.first, ingredient.second);
            break;
        }
            
        case ClientPacketType::PLAYER_MOVING_TYPE: {
            int moving_type;
            event.packet >> moving_type;
            event.player->moving_type = (MovingType)moving_type;
            sf::Packet moving_packet;
            moving_packet << ServerPacketType::PLAYER_MOVING_TYPE << moving_type << event.player->id;
            networking->sendToEveryone(moving_packet);
            break;
        }

        case ClientPacketType::PLAYER_JUMPED: {
            sf::Packet jumped_packet;
            jumped_packet << ServerPacketType::PLAYER_JUMPED << event.player->id;
            networking->sendToEveryone(jumped_packet);
            break;
        }
            
        case ClientPacketType::ITEM_DROP: {
            ItemType* dropped_item = event.player->inventory.getSelectedSlot().type;
            if(dropped_item != &items->nothing) {
                event.player->inventory.decreaseStack(event.player->inventory.selected_slot, 1);
                Item* dropped_item_instance = items->spawnItem(dropped_item, event.player->getX() + (event.player->flipped ? -1 : 1) * 10, event.player->getY());
                entities->addVelocityX(dropped_item_instance, (event.player->flipped ? -1 : 1) * 50);
            }
            break;
        }
        default:;
    }
}

void ServerPlayer::onEvent(InventoryItemChangeEvent& event) {
    ItemStack item = inventory.getItem(event.item_pos);
    sf::Packet packet;
    packet << ServerPacketType::INVENTORY << item.stack << item.type->id << (int)event.item_pos;
    connection->send(packet);
}

BlockBehaviour*& ServerPlayers::getBlockBehaviour(BlockType* type) {
    return blocks_behaviour[type->id];
}

ServerPlayer::~ServerPlayer() {
    inventory.item_change_event.removeListener(this);
}

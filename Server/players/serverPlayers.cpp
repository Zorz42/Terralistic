#include "serverPlayers.hpp"
#include "content.hpp"
#include <cstring>

void AirBehaviour::onRightClick(int x, int y, ServerPlayer* player) {
    BlockType* places_block = player->inventory.getSelectedSlot().type->places_block;
    if(places_block != &blocks->air) {
        bool can_place = true;
        for(int x_ = x; x_ < x + places_block->width; x_++)
            for(int y_ = y; y_ < y + places_block->height; y_++)
                if(blocks->getBlockType(x_, y_) != &blocks->air)
                    can_place = false;
        
        if(blocks->getBlockType(x, y + 1)->transparent && blocks->getBlockType(x, y - 1)->transparent && blocks->getBlockType(x + 1, y)->transparent && blocks->getBlockType(x - 1, y)->transparent && walls->getWallType(x, y) == &walls->clear)
            can_place = false;
            
        if(can_place && player->inventory.decreaseStack(player->inventory.selected_slot, 1))
           blocks->setBlockType(x, y, places_block);
    }
}

void ServerPlayers::init() {
    blocks->block_update_event.addListener(this);
    networking->new_connection_event.addListener(this);
    networking->connection_welcome_event.addListener(this);
    packet_event.addListener(this);
    networking->disconnect_event.addListener(this);
    world_saver->world_load_event.addListener(this);
    world_saver->world_save_event.addListener(this);
    entities->entity_absolute_velocity_change_event.addListener(this);
}

void ServerPlayers::postInit() {
    blocks_behaviour = new BlockBehaviour*[blocks->getNumBlockTypes()];
    for(int i = 0; i < blocks->getNumBlockTypes(); i++)
        blocks_behaviour[i] = &default_behaviour;
    
    getBlockBehaviour(&blocks->air) = &air_behaviour;
}

void ServerPlayers::stop() {
    blocks->block_update_event.removeListener(this);
    networking->new_connection_event.removeListener(this);
    networking->connection_welcome_event.removeListener(this);
    packet_event.removeListener(this);
    networking->disconnect_event.removeListener(this);
    world_saver->world_load_event.removeListener(this);
    world_saver->world_save_event.removeListener(this);
    entities->entity_absolute_velocity_change_event.removeListener(this);
    
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
    throw Exception("Could not find player by name");
}

ServerPlayer* ServerPlayers::getPlayerById(const int id) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entities->getEntities()[i];
            if(player->id == id)
                return player;
        }
    throw Exception("Could not find player by name");
}

bool ServerPlayers::playerExists(const std::string& name) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entities->getEntities()[i];
            if(player->name == name)
                return true;
        }
    return false;
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
        player_data->health = 100;
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
    player_data->health = player->health;
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
        if(type->ghost && player->inventory.getSelectedSlot().type->tool_powers.count(&walls->hammer))
            walls->startBreakingWall(x, y);
        
        if(type->effective_tool == &blocks->hand || (player->inventory.getSelectedSlot().type->tool_powers.count(type->effective_tool) && player->inventory.getSelectedSlot().type->tool_powers[type->effective_tool] >= type->required_tool_power))
            getBlockBehaviour(type)->onLeftClick(x, y, player);
        
        if(blocks->getBlockType(x, y) == type)
            break;
    }
}

void ServerPlayers::rightClickEvent(ServerPlayer* player, int x, int y) {
    WallType* places_wall = player->inventory.getSelectedSlot().type->places_wall;
    if(places_wall != &walls->clear) {
        if(walls->getWallType(x, y) == &walls->clear && player->inventory.decreaseStack(player->inventory.selected_slot, 1))
            walls->setWallType(x, y, places_wall);
    } else
        getBlockBehaviour(blocks->getBlockType(x, y))->onRightClick(x, y, player);
}

void ServerPlayers::fromSerial(const std::vector<char> &serial) {
    int iter = 0;
    while(iter < serial.size()) {
        ServerPlayerData* new_player = new ServerPlayerData(items, recipes);
        
        int inventory_serial_size = 3 * INVENTORY_SIZE;
        new_player->inventory.fromSerial(std::vector<char>(serial.begin() + iter, serial.begin() + iter + inventory_serial_size));
        iter += inventory_serial_size;
        
        memcpy(&new_player->x, &serial[iter], sizeof(int));
        iter += sizeof(int);
        
        memcpy(&new_player->y, &serial[iter], sizeof(int));
        iter += sizeof(int);
        
        while(serial[iter])
            new_player->name.push_back(serial[iter++]);
        iter++;

        memcpy(&new_player->health, &serial[iter], sizeof(int));
        iter += sizeof(int);
        
        all_players.push_back(new_player);
    }
}

std::vector<char> ServerPlayers::toSerial() {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER)
            savePlayer((ServerPlayer*)entities->getEntities()[i]);
    
    std::vector<char> serial;
    for(int i = 0; i < all_players.size(); i++) {
        std::vector<char> inventory_serial = all_players[i]->inventory.toSerial();
        serial.insert(serial.end(), inventory_serial.begin(), inventory_serial.end());
        
        serial.insert(serial.end(), {0, 0, 0, 0});
        memcpy(&serial[serial.size() - 4], &all_players[i]->x, sizeof(int));
        
        serial.insert(serial.end(), {0, 0, 0, 0});
        memcpy(&serial[serial.size() - 4], &all_players[i]->y, sizeof(int));
        
        serial.insert(serial.end(), all_players[i]->name.begin(), all_players[i]->name.end());
        serial.insert(serial.end(), 0);

        serial.insert(serial.end(), {0, 0, 0, 0});
        memcpy(&serial[serial.size() - 4], &all_players[i]->health, sizeof(int));
    }
    return serial;
}

void ServerPlayers::onEvent(BlockUpdateEvent& event) {
    getBlockBehaviour(blocks->getBlockType(event.x, event.y))->onUpdate(event.x, event.y);
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

    if(playerExists(player_name)) {
        ServerPlayer *already_joined_player = getPlayerByName(player_name);
        networking->kickConnection(already_joined_player->getConnection(), "You logged in from another location!");
    }
    
    ServerPlayer* player = addPlayer(player_name);
    player->setConnection(event.connection);

    sf::Packet healthPacket;
    healthPacket << WelcomePacketType::HEALTH << player->health;
    event.connection->sendDirectly(healthPacket);
    event.connection->send(std::vector<char>());
    
    sf::Packet packet;
    packet << WelcomePacketType::INVENTORY;
    event.connection->sendDirectly(packet);
    
    event.connection->send(player->inventory.toSerial());
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
                    int distance_x = item->getX() + ITEM_WIDTH / 2 - player->getX() - PLAYER_WIDTH / 2;
                    int distance_y = item->getY() + ITEM_WIDTH / 2 - player->getY() - PLAYER_HEIGHT / 2;
                    
                    if(abs(distance_x) < 50 && abs(distance_y) < 50) {
                        entities->addVelocityX(item, -distance_x * frame_length / 200.f);
                        entities->addVelocityY(item, -distance_y * frame_length / 30.f);
                    }
                    
                    if(abs(distance_x) < 5 && abs(distance_y) < PLAYER_HEIGHT / 2 && player->inventory.addItem(item->getType(), 1) != -1)
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
            
            if(event.player->breaking) {
                blocks->stopBreakingBlock(event.player->breaking_x, event.player->breaking_y);
                walls->stopBreakingWall(event.player->breaking_x, event.player->breaking_y);
            }
            
            event.player->breaking_x = breaking_x;
            event.player->breaking_y = breaking_y;
            event.player->breaking = true;
            leftClickEvent(event.player, breaking_x, breaking_y);
            break;
        }

        case ClientPacketType::STOPPED_BREAKING: {
            event.player->breaking = false;
            blocks->stopBreakingBlock(event.player->breaking_x, event.player->breaking_y);
            walls->stopBreakingWall(event.player->breaking_x, event.player->breaking_y);
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
            
            for(auto ingredient : recipe_crafted->ingredients)
                event.player->inventory.removeItem(ingredient.first, ingredient.second);
            event.player->inventory.addItem(recipe_crafted->result.type, recipe_crafted->result.stack);
            
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
                Item* dropped_item_instance = items->spawnItem(dropped_item, event.player->getX() + PLAYER_WIDTH / 2 + (event.player->flipped ? -1 : 1) * 10 - ITEM_WIDTH / 2, event.player->getY() + 10);
                entities->addVelocityX(dropped_item_instance, (event.player->flipped ? -1 : 1) * 40);
            }
            break;
        }
        case ClientPacketType::MAIN_PLAYER_POSITION: {
            int x, y;
            event.packet >> x >> y;
            
            if(abs(event.player->getX() - x) + abs(event.player->getY() - y) > 50) {
                sf::Packet packet;
                packet << ServerPacketType::MAIN_PLAYER_POSITION << event.player->getX() << event.player->getY();
                event.player->getConnection()->send(packet);
            } else {
                entities->setX(event.player, x);
                entities->setY(event.player, y);
            }
            break;
        }
        case ClientPacketType::PLAYER_RESPAWN: {
            setPlayerHealth(event.player, 40);
            addPlayer(event.player->name);
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

void ServerPlayers::setPlayerHealth(ServerPlayer* player, int health) {
    player->health = health;
    sf::Packet packet;
    packet << ServerPacketType::HEALTH << health;
    player->getConnection()->send(packet);
    
    if(health <= 0) {
        //empty inventory
        savePlayer(player);
        entities->removeEntity((Entity*)player);
    }
}

BlockBehaviour*& ServerPlayers::getBlockBehaviour(BlockType* type) {
    return blocks_behaviour[type->id];
}

ServerPlayer::~ServerPlayer() {
    inventory.item_change_event.removeListener(this);
}

void ServerPlayers::onEvent(WorldSaveEvent &event) {
    world_saver->setSectionData("players", toSerial());
}

void ServerPlayers::onEvent(WorldLoadEvent &event) {
    fromSerial(world_saver->getSectionData("players"));
}

void ServerPlayers::onEvent(EntityAbsoluteVelocityChangeEvent &event) {
    ServerPlayer* player = (ServerPlayer*)event.entity;
    int delta_vel_x = std::abs(player->getVelocityX() - event.old_vel_x);
    int delta_vel_y = std::abs(player->getVelocityY() - event.old_vel_y);
    if(delta_vel_x + delta_vel_y > 69) {
        int health_decrease = delta_vel_x + delta_vel_y - 69;
        setPlayerHealth(player, player->health - health_decrease);
    }
}

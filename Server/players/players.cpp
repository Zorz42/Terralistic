#include "serverPlayers.hpp"
#include "blocks.hpp"
#include <filesystem>
#include <fstream>
#include <utility>

static bool isBlockTree(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == BlockType::WOOD || blocks->getBlockType(x, y) == BlockType::LEAVES);
}

static bool isBlockWood(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == BlockType::WOOD;
}

static bool isBlockLeaves(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == BlockType::LEAVES;
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

ServerPlayers::ServerPlayers(Blocks* blocks, Entities* entities, Items* items, ServerNetworkingManager* networking_manager) : blocks(blocks), entities(entities), items(items), networking_manager(networking_manager) {
    custom_block_events[(int)BlockType::WOOD].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockType::LEAVES].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick = [](Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) {
        blocks->setBlockType(x, y, BlockType::DIRT);
    };

    custom_block_events[(int)BlockType::AIR].onRightClick = [](Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) {
        BlockType type = ::getItemInfo(player->inventory.getSelectedSlot().type).places;
        if(type != BlockType::AIR && player->inventory.decreaseStack(player->inventory.selected_slot, 1)) {
            blocks->setBlockType(x, y, type);
        }
    };

    custom_block_events[(int)BlockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick;
    
    custom_block_events[(int)BlockType::STONE].onUpdate = &stoneUpdate;
}

ServerPlayers::~ServerPlayers() {
    for(ServerPlayerData* i : all_players)
        delete i;
}

void ServerPlayers::init() {
    blocks->block_change_event.addListener(this);
    networking_manager->new_connection_event.addListener(this);
    networking_manager->connection_welcome_event.addListener(this);
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

void ServerPlayers::updatePlayersBreaking(unsigned short tick_length) {
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* player = (ServerPlayer*)entity;
            if(player->breaking)
                leftClickEvent(player, player->breaking_x, player->breaking_y, tick_length);
        }
}

void ServerPlayers::lookForItemsThatCanBePickedUp() {
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

void ServerPlayers::leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y, unsigned short tick_length) {
    if(custom_block_events[(int)blocks->getBlockType(x, y)].onLeftClick)
        custom_block_events[(int)blocks->getBlockType(x, y)].onLeftClick(blocks, x, y, player);
    else if(blocks->getBlockInfo(x, y).break_time != UNBREAKABLE)
        blocks->setBreakProgress(x, y, blocks->getBreakProgress(x, y) + tick_length);
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
    
    x = *(int*)iter;
    iter += 4;
    y = *(int*)iter;
    iter += 4;
    
    while(*iter)
        name.push_back(*iter++);
    iter++;
}

bool ServerPlayer::isColliding(Blocks* blocks) {
    return isCollidingWithBlocks(blocks) ||
    (
     moving_type == MovingType::SNEAK_WALKING && isCollidingWithBlocks(blocks, getX(), getY() + 1) &&
     (!isCollidingWithBlocks(blocks, getX() + 1, getY() + 1) || !isCollidingWithBlocks(blocks, getX() - 1, getY() + 1))
     );
}

void ServerPlayerData::serialize(std::vector<char>& serial) const {
    inventory.serialize(serial);
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = x;
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = y;
    
    serial.insert(serial.end(), name.begin(), name.end() + 1);
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
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER) {
            ServerPlayer* curr_player = (ServerPlayer*)entity;
            sf::Packet join_packet;
            join_packet << PacketType::PLAYER_JOIN << curr_player->getX() << curr_player->getY() << curr_player->id << curr_player->name << (unsigned char)curr_player->moving_type;
            event.connection->send(join_packet);
        }
}

void ServerPlayers::onEvent(ServerConnectionWelcomeEvent& event) {
    std::string player_name;
    event.client_welcome_packet >> player_name;
    ServerPlayer* player = addPlayer(player_name);
    player->connection = event.connection;
}

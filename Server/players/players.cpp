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

ServerPlayers::ServerPlayers(Blocks* blocks, Entities* entities, ServerItems* items) : blocks(blocks), entities(entities), items(items) {
    custom_block_events[(int)BlockType::WOOD].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockType::LEAVES].onUpdate = &treeUpdate;

    custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick = [](Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) {
        blocks->setBlockType(x, y, BlockType::DIRT);
    };

    custom_block_events[(int)BlockType::AIR].onRightClick = [](Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) {
        BlockType type = player->inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != BlockType::AIR && player->inventory.inventory_arr[player->inventory.selected_slot].decreaseStack(1)) {
            blocks->setBlockType(x, y, type);
        }
    };

    custom_block_events[(int)BlockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick;
    
    custom_block_events[(int)BlockType::STONE].onUpdate = &stoneUpdate;
}

ServerPlayers::~ServerPlayers() {
    for(ServerPlayer* i : all_players)
        delete i;
}

void ServerPlayers::init() {
    blocks->block_change_event.addListener(this);
}

ServerPlayer* ServerPlayers::getPlayerByName(const std::string& name) {
    for(ServerPlayer* player : all_players)
        if(player->name == name)
            return player;
    return nullptr;
}

ServerPlayer* ServerPlayers::addPlayer(const std::string& name) {
    ServerPlayer* player = getPlayerByName(name);
    
    if(!player) {
        int spawn_x = blocks->getWidth() / 2 * BLOCK_WIDTH * 2;
        
        int spawn_y = 0;
        for(unsigned short y = 0; y < blocks->getHeight(); y++) {
            if(!blocks->getBlockInfo(blocks->getWidth() / 2, y).transparent || !blocks->getBlockInfo(blocks->getWidth() / 2 + 1, y).transparent)
                break;
            spawn_y += BLOCK_WIDTH * 2;
        }
        
        player = new ServerPlayer(this, spawn_x, spawn_y - BLOCK_WIDTH * 6, name);
        all_players.emplace_back(player);
        player->sight_x = player->getX();
        player->sight_y = player->getY();
    }
    
    online_players.push_back(player);
    entities->registerEntity(player);
    return player;
}

void ServerPlayers::removePlayer(ServerPlayer* player) {
    online_players.erase(std::find(online_players.begin(), online_players.end(), player));
    entities->removeEntity(player);
}

void ServerPlayers::updatePlayersBreaking(unsigned short tick_length) {
    for(ServerPlayer* player : online_players)
        if(player->breaking)
            leftClickEvent(player, player->breaking_x, player->breaking_y, tick_length);
}

void ServerPlayers::lookForItemsThatCanBePickedUp() {
    for(ServerItem* item : items->getItems())
        for(ServerPlayer* player : online_players)
            if(abs(item->getX() + BLOCK_WIDTH - player->getX() - 14) < 50 && abs(item->getY() + BLOCK_WIDTH - player->getY() - 25) < 50 &&
               player->inventory.addItem(item->getType(), 1) != -1
               ) {
                items->removeItem(item);
            }
}

void ServerPlayers::updateBlocksInVisibleAreas() {
    for(ServerPlayer* player : online_players) {
        int start_x = (int)player->getSightBeginX() - 20, start_y = (int)player->getSightBeginY() - 20, end_x = player->getSightEndX() + 20, end_y = player->getSightEndY() + 20;
        if(start_x < 0)
            start_x = 0;
        if(start_y < 0)
            start_y = 0;
        if(end_x > blocks->getWidth())
            end_x = blocks->getWidth();
        if(end_y > blocks->getHeight())
            end_y = blocks->getHeight();
        
        bool finished = false;
        while(!finished) {
            finished = true;
            for(int y = start_y; y < end_y; y++)
                for(int x = start_x; x < end_x; x++) {
                    //ServerBlock curr_block = blocks->getBlock(x, y);
                    /*if(curr_block.hasScheduledLightUpdate()) {
                        curr_block.lightUpdate();
                        finished = false;
                    }*/
                    /*if(curr_block.getLiquidType() != LiquidType::EMPTY && curr_block.canUpdateLiquid()) {
                        curr_block.liquidUpdate();
                        finished = false;
                    }*/
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
    all_players.emplace_back(new ServerPlayer(this, iter));
    return iter;
}

ServerPlayer::ServerPlayer(ServerPlayers* players, char*& iter) : Entity(EntityType::PLAYER, *(int*)iter, *(int*)(iter + 4)), inventory(players) {
    friction = false;
    iter += 8;
    
    for(InventoryItem& i : inventory.inventory_arr)
        iter = i.loadFromSerial(iter);
    
    while(*iter)
        name.push_back(*iter++);
    iter++;
    
    sight_x = getX();
    sight_y = getY();
}

bool ServerPlayer::isColliding(Blocks* blocks) {
    return isCollidingWithBlocks(blocks) ||
    (
     moving_type == MovingType::SNEAK_WALKING && isCollidingWithBlocks(blocks, getX(), getY() + 1) &&
     (!isCollidingWithBlocks(blocks, getX() + 1, getY() + 1) || !isCollidingWithBlocks(blocks, getX() - 1, getY() + 1))
     );
}

void ServerPlayer::serialize(std::vector<char>& serial) const {
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = getX();
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = getY();
    
    for(const InventoryItem& i : inventory.inventory_arr)
        i.serialize(serial);
    
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

unsigned short ServerPlayer::getSightBeginX() const {
    return sight_x / (BLOCK_WIDTH * 2) - sight_width / 2;
}

unsigned short ServerPlayer::getSightEndX() const {
    return sight_x / (BLOCK_WIDTH * 2) + sight_width / 2;
}

unsigned short ServerPlayer::getSightBeginY() const {
    return sight_y / (BLOCK_WIDTH * 2) - sight_height / 2;
}

unsigned short ServerPlayer::getSightEndY() const {
    return sight_y / (BLOCK_WIDTH * 2) + sight_height / 2;
}

#include "serverPlayers.hpp"
#include "serverBlocks.hpp"
#include <filesystem>
#include <fstream>
#include <utility>

static bool isBlockTree(ServerBlock block) {
    return block.refersToABlock() && (block.getBlockType() == BlockType::WOOD || block.getBlockType() == BlockType::LEAVES);
}

static bool isBlockWood(ServerBlock block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::WOOD;
}

static bool isBlockLeaves(ServerBlock block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::LEAVES;
}

ServerPlayers::ServerPlayers(ServerBlocks* blocks, ServerEntities* entities, ServerItems* items) : blocks(blocks), entities(entities), items(items) {
    custom_block_events[(int)BlockType::WOOD].onUpdate = [](ServerBlocks* server_blocks, ServerBlock* this_block) {
        ServerBlock upper, lower, left, right;
        if(this_block->getY() != 0)
            upper = server_blocks->getBlock(this_block->getX(), this_block->getY() - 1);
        if(this_block->getY() != server_blocks->getHeight() - 1)
            lower = server_blocks->getBlock(this_block->getX(), this_block->getY() + 1);
        if(this_block->getX() != 0)
            left = server_blocks->getBlock(this_block->getX() - 1, this_block->getY());
        if(this_block->getX() != server_blocks->getWidth() - 1)
            right = server_blocks->getBlock(this_block->getX() + 1, this_block->getY());
        
        if(
           (!isBlockTree(lower) && !isBlockTree(left) && !isBlockTree(right)) ||
           (isBlockWood(upper) && isBlockWood(right) && !isBlockTree(left) && !isBlockTree(lower)) ||
           (isBlockWood(upper) && isBlockWood(left) && !isBlockTree(right) && !isBlockTree(lower)) ||
           (isBlockLeaves(left) && !isBlockTree(right) && !isBlockTree(upper) && !isBlockTree(lower)) ||
           (isBlockLeaves(right) && !isBlockTree(left) && !isBlockTree(upper) && !isBlockTree(lower)) ||
           (!isBlockTree(lower) && isBlockLeaves(left) && isBlockLeaves(right) && isBlockLeaves(upper))
           )
            this_block->breakBlock();
    };

    custom_block_events[(int)BlockType::LEAVES].onUpdate = custom_block_events[(int)BlockType::WOOD].onUpdate;

    custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick = [](ServerBlock* this_block, ServerPlayer* peer) {
        this_block->setType(BlockType::DIRT);
    };

    custom_block_events[(int)BlockType::AIR].onRightClick = [](ServerBlock* this_block, ServerPlayer* peer) {
        BlockType type = peer->inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != BlockType::AIR && peer->inventory.inventory_arr[peer->inventory.selected_slot].decreaseStack(1)) {
            this_block->setType(type);
            this_block->update();
        }
    };

    custom_block_events[(int)BlockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick;
}

ServerPlayers::~ServerPlayers() {
    for(ServerPlayer* i : all_players)
        delete i;
}

void ServerPlayers::init() {
    blocks->block_update_event.addListener(this);
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
        player = new ServerPlayer(entities, blocks->getSpawnX(), blocks->getSpawnY() - BLOCK_WIDTH * 4, name);
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
            leftClickEvent(blocks->getBlock(player->breaking_x, player->breaking_y), player, tick_length);
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
                    ServerBlock curr_block = blocks->getBlock(x, y);
                    if(curr_block.hasScheduledLightUpdate()) {
                        curr_block.lightUpdate();
                        finished = false;
                    }
                    if(curr_block.getLiquidType() != LiquidType::EMPTY && curr_block.canUpdateLiquid()) {
                        curr_block.liquidUpdate();
                        finished = false;
                    }
                }
        }
    }
}

void ServerPlayers::leftClickEvent(ServerBlock this_block, ServerPlayer* peer, unsigned short tick_length) {
    if(custom_block_events[(int)this_block.getBlockType()].onLeftClick)
        custom_block_events[(int)this_block.getBlockType()].onLeftClick(&this_block, peer);
    else if(this_block.getBlockInfo().break_time != UNBREAKABLE) {
        this_block.setBreakProgress(this_block.getBreakProgress() + tick_length);
        if(this_block.getBreakProgress() >= this_block.getBlockInfo().break_time)
            this_block.breakBlock();
    }
}

void ServerPlayers::rightClickEvent(ServerBlock this_block, ServerPlayer* peer) {
    if(custom_block_events[(int)this_block.getBlockType()].onRightClick)
        custom_block_events[(int)this_block.getBlockType()].onRightClick(&this_block, peer);
}

char* ServerPlayers::addPlayerFromSerial(char* iter) {
    all_players.emplace_back(new ServerPlayer(entities, iter));
    return iter;
}

ServerPlayer::ServerPlayer(ServerEntities* entities, char*& iter) : ServerEntity(entities, *(int*)iter, *(int*)(iter + 4)) {
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

bool ServerPlayer::isColliding(ServerBlocks* blocks) {
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

void ServerPlayers::onEvent(ServerBlockUpdateEvent& event) {
    if(custom_block_events[(int)event.block.getBlockType()].onUpdate)
        custom_block_events[(int)event.block.getBlockType()].onUpdate(blocks, &event.block);
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

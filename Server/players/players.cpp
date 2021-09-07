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

Players::Players(ServerBlocks* blocks, ServerEntityManager* entity_manager) : blocks(blocks), entity_manager(entity_manager) {
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

Players::~Players() {
    for(ServerPlayer* i : all_players)
        delete i;
}

ServerPlayer* Players::getPlayerByName(const std::string& name) {
    for(ServerPlayer* player : all_players)
        if(player->name == name)
            return player;
    return nullptr;
}

ServerPlayer* Players::addPlayer(const std::string& name) {
    ServerPlayer* player = getPlayerByName(name);
    
    if(!player) {
        player = new ServerPlayer(name);
        all_players.emplace_back(player);
        player->x = blocks->getSpawnX();
        player->y = blocks->getSpawnY() - BLOCK_WIDTH * 4;
        player->sight_x = player->x;
        player->sight_y = player->y;
    }
    
    online_players.push_back(player);
    return player;
}

void Players::removePlayer(ServerPlayer* player) {
    for(int i = 0; i < online_players.size(); i++)
        if(player == online_players[i]) {
            online_players.erase(online_players.begin() + i);
            break;
        }
}

void Players::updatePlayersBreaking(unsigned short tick_length) {
    for(ServerPlayer* player : online_players)
        if(player->breaking)
            leftClickEvent(blocks->getBlock(player->breaking_x, player->breaking_y), player, tick_length);
}

void Players::lookForItemsThatCanBePickedUp() {
    for(int i = 0; i < entity_manager->getEntities().size(); i++)
        for(ServerPlayer* player : online_players)
            if(entity_manager->getEntities()[i]->type == EntityType::ITEM &&
               abs(entity_manager->getEntities()[i]->getX() + BLOCK_WIDTH - player->x - 14) < 50 && abs(entity_manager->getEntities()[i]->getY() + BLOCK_WIDTH - player->y - 25) < 50 &&
               player->inventory.addItem(((ServerItem*)entity_manager->getEntities()[i])->getType(), 1) != -1
               )
                entity_manager->removeEntity(entity_manager->getEntities()[i]);
}

void Players::updateBlocksInVisibleAreas() {
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

void Players::leftClickEvent(ServerBlock this_block, ServerPlayer* peer, unsigned short tick_length) {
    if(custom_block_events[(int)this_block.getBlockType()].onLeftClick)
        custom_block_events[(int)this_block.getBlockType()].onLeftClick(&this_block, peer);
    else if(this_block.getBlockInfo().break_time != UNBREAKABLE) {
        this_block.setBreakProgress(this_block.getBreakProgress() + tick_length);
        if(this_block.getBreakProgress() >= this_block.getBlockInfo().break_time)
            this_block.breakBlock();
    }
}

void Players::rightClickEvent(ServerBlock this_block, ServerPlayer* peer) {
    if(custom_block_events[(int)this_block.getBlockType()].onRightClick)
        custom_block_events[(int)this_block.getBlockType()].onRightClick(&this_block, peer);
}

char* Players::addPlayerFromSerial(char* iter) {
    all_players.push_back(new ServerPlayer(iter));
    return iter;
}

ServerPlayer::ServerPlayer(char*& iter) : id(curr_id++) {
    for(InventoryItem& i : inventory.inventory_arr)
        iter = i.loadFromSerial(iter);
    
    x = *(int*)iter;
    iter += 4;
    y = *(int*)iter;
    iter += 4;
    
    do
        name.push_back(*iter++);
    while(*iter);
    
    sight_x = x;
    sight_y = y;
}

void ServerPlayer::serialize(std::vector<char>& serial) const {
    for(const InventoryItem& i : inventory.inventory_arr)
        i.serialize(serial);
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = x;
    
    serial.insert(serial.end(), {0, 0, 0, 0});
    *(int*)&serial[serial.size() - 4] = y;
    
    serial.insert(serial.end(), name.begin(), name.end() + 1);
}

void Players::onEvent(ServerBlockUpdateEvent& event) {
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

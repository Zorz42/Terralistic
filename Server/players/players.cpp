#include "players.hpp"
#include "blocks.hpp"
#include <filesystem>
#include <fstream>

static bool isBlockTree(Block block) {
    return block.refersToABlock() && (block.getBlockType() == BlockType::WOOD || block.getBlockType() == BlockType::LEAVES);
}

static bool isBlockWood(Block block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::WOOD;
}

static bool isBlockLeaves(Block block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::LEAVES;
}

Players::Players(Blocks* parent_blocks, Items* parent_items) : blocks(parent_blocks), items(parent_items) {
    custom_block_events[(int)BlockType::WOOD].onUpdate = [](Blocks* server_blocks, Block* this_block) {
        Block upper, lower, left, right;
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

    custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick = [](Block* this_block, Player* peer) {
        this_block->setType(BlockType::DIRT);
    };

    custom_block_events[(int)BlockType::AIR].onRightClick = [](Block* this_block, Player* peer) {
        BlockType type = peer->inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != BlockType::AIR && peer->inventory.inventory_arr[peer->inventory.selected_slot].decreaseStack(1)) {
            this_block->setType(type);
            this_block->update();
        }
    };

    custom_block_events[(int)BlockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick;
}

Players::~Players() {
    for(Player* i : all_players)
        delete i;
}

Player* Players::getPlayerByName(const std::string& name) {
    for(Player* player : all_players)
        if(player->name == name)
            return player;
    return nullptr;
}

Player* Players::addPlayer(const std::string& name) {
    Player* player = getPlayerByName(name);
    
    if(!player) {
        player = new Player(name);
        all_players.emplace_back(player);
        player->y = blocks->getSpawnY() - BLOCK_WIDTH * 2;
        player->x = blocks->getSpawnX();
    }
    
    online_players.push_back(player);
    return player;
}

void Players::removePlayer(Player* player) {
    for(int i = 0; i < online_players.size(); i++)
        if(player == online_players[i]) {
            online_players.erase(online_players.begin() + i);
            break;
        }
}

void Players::updatePlayersBreaking(unsigned short tick_length) {
    for(Player* player : online_players)
        if(player->breaking)
            leftClickEvent(blocks->getBlock(player->breaking_x, player->breaking_y), player, tick_length);
}

void Players::lookForItemsThatCanBePickedUp() {
    for(const Item& i : items->getItems())
        for(Player* player : online_players)
            if(abs(i.getX() / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(i.getY() / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50)
                if(player->inventory.addItem(i.getType(), 1) != -1)
                    items->removeItem(i);
}

void Players::updateBlocksInVisibleAreas() {
    for(Player* player : online_players) {
        int start_x = player->getSightBeginX(), start_y = player->getSightBeginY(), end_x = player->getSightEndX(), end_y = player->getSightEndY();
        
        bool finished = false;
        while(!finished) {
            finished = true;
            for(unsigned short y = start_y; y < end_y; y++)
                for(unsigned short x = start_x; x < end_x; x++) {
                    Block curr_block = blocks->getBlock(x, y);
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
        
        for(unsigned short y = start_y; y < end_y; y++)
            for(unsigned short x = start_x; x < end_x; x++) {
                Block curr_block = blocks->getBlock(x, y);
                if(curr_block.hasLightChanged()) {
                    curr_block.markLightUnchanged();
                    ServerLightChangeEvent event(curr_block);
                    event.call();
                }
            }
    }
}

void Players::leftClickEvent(Block this_block, Player* peer, unsigned short tick_length) {
    if(custom_block_events[(int)this_block.getBlockType()].onLeftClick)
        custom_block_events[(int)this_block.getBlockType()].onLeftClick(&this_block, peer);
    else if(this_block.getUniqueBlock().break_time != UNBREAKABLE) {
        this_block.setBreakProgress(this_block.getBreakProgress() + tick_length);
        if(this_block.getBreakProgress() >= this_block.getUniqueBlock().break_time)
            this_block.breakBlock();
    }
}

void Players::rightClickEvent(Block this_block, Player* peer) {
    if(custom_block_events[(int)this_block.getBlockType()].onRightClick)
        custom_block_events[(int)this_block.getBlockType()].onRightClick(&this_block, peer);
}

Player* Players::addPlayerFromFile(const std::string& path) {
    Player* player = new Player(path, path.substr(path.find_last_of('/') + 1, path.size() - 1));
    all_players.push_back(player);
    return player;
}

Player::Player(const std::string& path, const std::string& name) : id(curr_id++), name(name) {
    std::ifstream data_file(path, std::ios::binary);
    for(auto & i : inventory.inventory_arr) {
        char c;
        data_file >> c;
        i.setTypeWithoutProcessing((ItemType)c);

        unsigned short stack;
        data_file.read((char*)&stack, sizeof(stack));
        i.setStackWithoutProcessing(stack);
    }

    data_file.read((char*)&x, sizeof(x));
    data_file.read((char*)&y, sizeof(y));
}

void Player::saveTo(std::string path) const {
    std::ofstream data_file(path, std::ios::binary);
    for(const auto& i : inventory.inventory_arr) {
        data_file << (char)i.getType();
        unsigned short stack = i.getStack();
        data_file.write((char*)&stack, sizeof(stack));
    }
    data_file.write((char*)&x, sizeof(x));
    data_file.write((char*)&y, sizeof(y));
    data_file.close();
}

void Players::onEvent(ServerBlockUpdateEvent& event) {
    if(custom_block_events[(int)event.block.getBlockType()].onUpdate)
        custom_block_events[(int)event.block.getBlockType()].onUpdate(blocks, &event.block);
}

unsigned short Player::getSightBeginX() {
    return x / 16 - sight_width / 2 - 20;
}

unsigned short Player::getSightEndX() {
    return x / 16 + sight_width / 2 + 20;
}

unsigned short Player::getSightBeginY() {
    return y / 16 - sight_height / 2 - 20;
}

unsigned short Player::getSightEndY() {
    return y / 16 + sight_height / 2 + 20;
}

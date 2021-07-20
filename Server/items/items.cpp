#include "items.hpp"
#include <random>
#include "properties.hpp"
#include "packetType.hpp"

void Items::updateItems(float frame_length) {
    for(auto & item : item_arr)
        item.update(frame_length);
}

void Items::spawnItem(ItemType item_id, int x, int y, short id) {
    static short curr_id = 0;
    if(id == -1)
        id = curr_id++;
    
    ServerItemCreationEvent event(item_id, x, y, id);
    event.call();
    
    if(event.cancelled)
        return;
    
    item_arr.emplace_back();
    item_arr.back().create(item_id, x, y, id, parent_blocks);
}

void Item::create(ItemType item_id_, int x_, int y_, unsigned short id_, Blocks* parent_blocks_) {
    static std::random_device device;
    static std::mt19937 engine(device());
    velocity_x = (int)engine() % 100;
    velocity_y = -int(engine() % 100) - 50;
    
    x = x_ * 100;
    y = y_ * 100;
    id = id_;
    item_id = item_id_;
    parent_blocks = parent_blocks_;
}

const ItemInfo& Item::getUniqueItem() const {
    return ::getItemInfo(item_id);
}

void Item::update(float frame_length) {
    int prev_x = x, prev_y = y;
    
    // move and go back if colliding
    velocity_y += (int)frame_length / 16.0f * 5.0f;
    
    for(int i = 0; i < frame_length / 16 * velocity_x; i++) {
        x++;
        if(collidingWithBlocks()) {
            x--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_x; i--) {
        x--;
        if(collidingWithBlocks()) {
            x++;
            break;
        }
    }
    for(int i = 0; i < frame_length / 16 * velocity_y; i++) {
        y++;
        if(collidingWithBlocks()) {
            y--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_y; i--) {
        y--;
        if(collidingWithBlocks()) {
            y++;
            break;
        }
    }
    
    y++;
    if(collidingWithBlocks())
        velocity_y = 0;
    y--;
    
    if(velocity_x > 0) {
        velocity_x -= frame_length / 8;
        if(velocity_x < 0)
            velocity_x = 0;
    }
    else if(velocity_x < 0) {
        velocity_x += frame_length / 8;
        if(velocity_x > 0)
            velocity_x = 0;
    }
    
    if(prev_x != x || prev_y != y) {
        ServerItemMovementEvent event(*this);
        event.call();
        
        if(event.cancelled) {
            x = prev_x;
            y = prev_y;
        }
    }
}

bool Item::collidingWithBlocks() const {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!parent_blocks->getBlock((unsigned short)(block_x + x_), (unsigned short)(block_y + y_)).getUniqueBlock().transparent)
                return true;
    return false;
}

void Items::destroyItem(const Item& item_to_destroy) {
    for(unsigned long i = 0; i < item_arr.size(); i++)
        if(item_arr[i].getId() == item_to_destroy.getId()) {
            ServerItemDeletionEvent event(item_arr[i]);
            event.call();
            
            if(event.cancelled)
                return;
            
            item_arr.erase(item_arr.begin() + i);
        }
}

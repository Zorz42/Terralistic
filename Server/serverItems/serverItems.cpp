#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

void ServerItems::updateItems(float frame_length) {
    for(auto & item : item_arr)
        item.update(frame_length);
}

void ServerItems::spawnItem(ItemType type, int x, int y) {
    static std::random_device device;
    static std::mt19937 engine(device());
    ServerItem item(parent_blocks, type, x, y);
    
    ServerItemCreationEvent event(type, x, y, item.getId());
    event.call();
    
    if(event.cancelled)
        return;
    
    item.addVelocityX(engine() % 100);
    item.addVelocityY(-(engine() % 100) - 50);
    
    item_arr.emplace_back(item);
}

const ItemInfo& ServerItem::getUniqueItem() const {
    return ::getItemInfo(type);
}

void ServerItem::addVelocityX(int vel_x) {
    velocity_x += vel_x;
}

void ServerItem::addVelocityY(int vel_y) {
    velocity_y += vel_y;
}

int ServerItem::getX() const {
    return x;
}

int ServerItem::getY() const {
    return y;
}

unsigned short ServerItem::getId() const {
    return id;
}

ItemType ServerItem::getType() const {
    return type;
}

void ServerItem::update(float frame_length) {
    int prev_x = x, prev_y = y;

    velocity_y += frame_length / 16.0f * 5.0f;
    
    int x_to_go = x + frame_length / 16.0f * velocity_x;
    while(x_to_go != x) {
        x += x_to_go > x ? 1 : -1;
        if(collidingWithBlocks()) {
            x -= x_to_go > x ? 1 : -1;
            break;
        }
    }
    
    int y_to_go = y + frame_length / 16.0f * velocity_y;
    while(y_to_go != y) {
        y += y_to_go > y ? 1 : -1;
        if(collidingWithBlocks()) {
            y -= y_to_go > y ? 1 : -1;
            break;
        }
    }
    
    grounded();
    
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

bool ServerItem::collidingWithBlocks() const {
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

bool ServerItem::grounded() {
    y++;
    bool is_grounded = collidingWithBlocks();
    y--;
    return is_grounded;
}

void ServerItems::removeItem(const ServerItem& item_to_remove) {
    for(unsigned long i = 0; i < item_arr.size(); i++)
        if(item_arr[i].getId() == item_to_remove.getId()) {
            ServerItemDeletionEvent event(item_arr[i]);
            event.call();
            
            if(event.cancelled)
                return;
            
            item_arr.erase(item_arr.begin() + i);
        }
}

void ServerItems::onEvent(ServerBlockBreakEvent& event) {
    if(event.block.getUniqueBlock().drop != ItemType::NOTHING)
        spawnItem(event.block.getUniqueBlock().drop, event.block.getX() * BLOCK_WIDTH, event.block.getY() * BLOCK_WIDTH);
}

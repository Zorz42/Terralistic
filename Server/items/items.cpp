#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

void ServerItems::updateItems(float frame_length) {
    for(auto & item : item_arr)
        item->update(frame_length);
}

void ServerItems::spawnItem(ItemType type, int x, int y) {
    static std::random_device device;
    static std::mt19937 engine(device());
    ServerItem* item = new ServerItem(parent_blocks, type, x, y);
    
    ServerItemCreationEvent event(type, x, y, item->getId());
    event.call();
    
    if(event.cancelled)
        return;
    
    //item->addVelocityX(engine() % 100 / 100.f);
    //item->addVelocityY(-((engine() % 100) - 50) / 100.f);
    
    item_arr.emplace_back(item);
}

const ItemInfo& ServerItem::getItemInfo() const {
    return ::getItemInfo(type);
}

void ServerItem::addVelocityX(float vel_x) {
    velocity_x += vel_x;
}

void ServerItem::addVelocityY(float vel_y) {
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

    velocity_y += frame_length / 1000.f;
    
    int x_to_go = x + frame_length / 16.f * velocity_x;
    while(x_to_go != x) {
        x += x_to_go > x ? 1 : -1;
        if(isColliding(parent_blocks)) {
            x -= x_to_go > x ? 1 : -1;
            break;
        }
    }
    
    int y_to_go = y + frame_length / 16.f * velocity_y;
    while(y_to_go != y) {
        y += y_to_go > y ? 1 : -1;
        if(isColliding(parent_blocks)) {
            y -= y_to_go > y ? 1 : -1;
            break;
        }
    }
    
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

bool ServerItem::grounded() {
    y++;
    bool is_grounded = isColliding(parent_blocks);
    y--;
    return is_grounded;
}

void ServerItems::removeItem(const ServerItem& item_to_remove) {
    for(unsigned long i = 0; i < item_arr.size(); i++)
        if(item_arr[i]->getId() == item_to_remove.getId()) {
            ServerItemDeletionEvent event(*item_arr[i]);
            event.call();
            
            if(event.cancelled)
                return;
            
            item_arr.erase(item_arr.begin() + i);
            delete &item_to_remove;
        }
}

void ServerItems::onEvent(ServerBlockBreakEvent& event) {
    if(event.block.getBlockInfo().drop != ItemType::NOTHING)
        spawnItem(event.block.getBlockInfo().drop, event.block.getX() * BLOCK_WIDTH * 2, event.block.getY() * BLOCK_WIDTH * 2);
}

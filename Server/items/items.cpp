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
    
    item->addVelocityX(int(engine() % 20) - 10);
    item->addVelocityY(-int(engine() % 10) - 10);
    
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

    velocity_y += frame_length / 20.f;
    
    float y_to_be = y + float(velocity_y * frame_length) / 100;
    float move_y = y_to_be - y;
    int y_factor = move_y > 0 ? 1 : -1;
    for(int i = 0; i < std::abs(move_y); i++) {
        y += y_factor;
        if(isColliding(parent_blocks)) {
            y -= y_factor;
            velocity_y = 0;
            break;
        }
    }
    if(velocity_y)
        y = y_to_be;
    
    float x_to_be = x + float(velocity_x * frame_length) / 100;
    float move_x = x_to_be - x;
    int x_factor = move_x > 0 ? 1 : -1;
    bool has_collided_x = false;
    bool has_moved_x = false;
    for(int i = 0; i < std::abs(move_x); i++) {
        x += x_factor;
        if(isColliding(parent_blocks)) {
            x -= x_factor;
            has_collided_x = true;
            break;
        }
        has_moved_x = true;
    }
    if(!has_collided_x)
        x = x_to_be;
    
    velocity_y *= 0.99f;
    velocity_x *= grounded() ? 0.9f : 0.99f;
    
    if(prev_x != int(x) || prev_y != int(y)) {
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

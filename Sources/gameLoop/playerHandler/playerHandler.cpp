
//
//  playerHandler.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#include "init.hpp"
#include "playerHandler.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "networkingModule.hpp"
#include "gameLoop.hpp"

#define INC_X playerHandler::position_x++;playerHandler::view_x++
#define DEC_X playerHandler::position_x--;playerHandler::view_x--
#define INC_Y playerHandler::position_y++;playerHandler::view_y++
#define DEC_Y playerHandler::position_y--;playerHandler::view_y--

// this handles player and its movement

bool key_up = false, jump = false;

ogl::rect inventory_slots[20], select_rect, under_text_rect(ogl::top_left);
ogl::texture stack_textures[20], mouse_stack_texture{ogl::top_left};

#define MARGIN 10

INIT_SCRIPT
    for(int i = 0; i < 20; i++) {
        inventory_slots[i].setOrientation(ogl::top);
        inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].setHeight(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setWidth(2 * BLOCK_WIDTH + MARGIN);
        inventory_slots[i].setY(MARGIN + i / 10 * 2 * MARGIN + i / 10 * 2 * BLOCK_WIDTH);
        inventory_slots[i].setX(short((i - 5 - i / 10 * 10) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    }
    
    select_rect.setOrientation(ogl::top);
    select_rect.setColor(50, 50, 50);
    select_rect.setWidth(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setHeight(2 * BLOCK_WIDTH + 2 * MARGIN);
    select_rect.setY(MARGIN / 2);
    
    under_text_rect.setColor(0, 0, 0);
    playerHandler::player.loadFromFile("texturePack/misc/player.png");
    playerHandler::player.scale = 2;
    
    for(int i = 0; i < 20; i++)
        stack_textures[i] = ogl::texture(ogl::top_left);
INIT_SCRIPT_END

void playerHandler::prepare() {
    position_x = blockEngine::world_width / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    position_y = blockEngine::world_height / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    view_x = position_x;
    view_y = position_y;
    
    selectSlot(0);
    player_inventory.open = false;
}

void playerHandler::handleEvents(SDL_Event& event) {
#define VELOCITY 20
#define JUMP_VELOCITY 80
    static bool key_left = false, key_right = false;
    if(event.type == SDL_KEYDOWN)
        switch (event.key.keysym.sym) {
            case SDLK_SPACE:
                if(!key_up) {
                    key_up = true;
                    jump = true;
                }
                break;            case SDLK_a:
                if(!key_left) {
                    key_left = true;
                    velocity_x -= VELOCITY;
                    player.flipped = true;
                }
                break;
            case SDLK_d:
                if(!key_right) {
                    key_right = true;
                    velocity_x += VELOCITY;
                    player.flipped = false;
                }
                break;
            default:;
        }
    else if(event.type == SDL_KEYUP)
        switch (event.key.keysym.sym) {
            case SDLK_SPACE:
                if(key_up) {
                    key_up = false;
                    jump = false;
                    if(velocity_y < -10)
                        velocity_y = -10;
                }
                break;
            case SDLK_a:
                key_left = false;
                velocity_x += VELOCITY;
                break;
            case SDLK_d:
                key_right = false;
                velocity_x -= VELOCITY;
                break;
            default:;
        }
    
    if(event.type == SDL_TEXTINPUT) {
        char c = event.text.text[0];
        if(c >= '0' && c <= '9') {
            if(c == '0')
                c = ':';
            selectSlot(c - '1');
        }
    } else if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_e) {
        player_inventory.open = !player_inventory.open;
        if(!player_inventory.open && player_inventory.getMouseItem()->item_id != itemEngine::NOTHING) {
            unsigned char result = player_inventory.addItem(player_inventory.getMouseItem()->item_id, player_inventory.getMouseItem()->getStack());
            player_inventory.clearMouseItem();
            packets::packet packet(packets::INVENTORY_SWAP);
            packet << result;
            networking::sendPacket(packet);
        }
    } else if(hovered && event.type == SDL_MOUSEBUTTONDOWN && event.button.button == SDL_BUTTON_LEFT) {
        player_inventory.swapWithMouseItem(hovered);
        packets::packet packet(packets::INVENTORY_SWAP);
        packet << (unsigned char)(hovered - &player_inventory.inventory[0]);
        networking::sendPacket(packet);
    }
}

bool isPlayerColliding() {
#define COLLISION_PADDING 2
    
    if(playerHandler::position_x < playerHandler::player.getWidth() / 2 || playerHandler::position_y < playerHandler::player.getHeight() / 2 ||
       playerHandler::position_y >= blockEngine::world_height * BLOCK_WIDTH - playerHandler::player.getHeight() / 2 ||
       playerHandler::position_x >= blockEngine::world_width * BLOCK_WIDTH - playerHandler::player.getWidth() / 2)
        return true;

    unsigned short begin_x = playerHandler::position_x / BLOCK_WIDTH - playerHandler::player.getWidth() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    unsigned short end_x = playerHandler::position_x / BLOCK_WIDTH + playerHandler::player.getWidth() / 2 / BLOCK_WIDTH + COLLISION_PADDING;

    unsigned short begin_y = playerHandler::position_y / BLOCK_WIDTH - playerHandler::player.getHeight() / 2 / BLOCK_WIDTH - COLLISION_PADDING;
    unsigned short end_y = playerHandler::position_y / BLOCK_WIDTH + playerHandler::player.getHeight() / 2 / BLOCK_WIDTH + COLLISION_PADDING;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > blockEngine::world_width)
        end_x = blockEngine::world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > blockEngine::world_height)
        end_y = blockEngine::world_height;
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(!blockEngine::getChunk(x >> 4, y >> 4).loaded || (swl::colliding({short(x * BLOCK_WIDTH - playerHandler::view_x + swl::window_width / 2), short(y * BLOCK_WIDTH - playerHandler::view_y + swl::window_height / 2), BLOCK_WIDTH, BLOCK_WIDTH}, playerHandler::player.getRect()) && !blockEngine::getBlock(x, y).getUniqueBlock().ghost))
                return true;
    return false;
}

bool touchingGround() {
    INC_Y;
    bool result = isPlayerColliding();
    DEC_Y;
    return result;
}

void playerHandler::move() {
    if(view_x < swl::window_width / 2)
        view_x = swl::window_width / 2;
    if(view_y < swl::window_height / 2)
        view_y = swl::window_height / 2;
    if(view_x >= blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2)
        view_x = blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2;
    if(view_y >= blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2)
        view_y = blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2;
    
    int move_x = velocity_x * gameLoop::frame_length / 100, move_y = velocity_y * gameLoop::frame_length / 100;
    
    for(int i = 0; i < move_x; i++) {
        INC_X;
        if(isPlayerColliding()) {
            DEC_X;
            break;
        }
    }
    for(int i = 0; i > move_x; i--) {
        DEC_X;
        if(isPlayerColliding()) {
            INC_X;
            break;
        }
    }
    for(int i = 0; i < move_y; i++) {
        INC_Y;
        if(isPlayerColliding()) {
            DEC_Y;
            break;
        }
    }
    for(int i = 0; i > move_y; i--) {
        DEC_Y;
        if(isPlayerColliding()) {
            INC_Y;
            if(velocity_y < 0)
                velocity_y = 0;
            break;
        }
    }
    if(touchingGround() && jump) {
        velocity_y = -JUMP_VELOCITY;
        jump = false;
    }
    view_x = position_x;
    view_y = position_y;
    
    if(gameLoop::online && (move_x || move_y)) {
        packets::packet packet(packets::PLAYER_MOVEMENT);
        packet << position_x << position_y << (char)player.flipped;
        networking::sendPacket(packet);
    }
}

void renderItem(inventory::inventoryItem* item, int x, int y, int i) {
    if(item->getUniqueItem().texture != nullptr)
        swl::render(item->getUniqueItem().texture, {(short)x, (short)y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
    if(item->getStack() > 1) {
        ogl::texture *stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        stack_texture->setX(short(x + BLOCK_WIDTH * 2 - stack_texture->getWidth()));
        stack_texture->setY(short(y + BLOCK_WIDTH * 2 - stack_texture->getHeight()));
        stack_texture->render();
    }
}

void updateStackTexture(int i) {
    inventory::inventoryItem* item = i == -1 ? playerHandler::player_inventory.getMouseItem() : &playerHandler::player_inventory.inventory[i];
    if(item->stack_changed) {
        ogl::texture* stack_texture = i == -1 ? &mouse_stack_texture : &stack_textures[i];
        if(item->getStack() > 1)
            stack_texture->loadFromText(std::to_string(item->getStack()), {255, 255, 255});
    }
}

void playerHandler::render() {
    player.setX(short(position_x - view_x));
    player.setY(short(position_y - view_y));
    player.render();
    
    select_rect.setX(short((player_inventory.selected_slot - 5) * (2 * BLOCK_WIDTH + 2 * MARGIN) + 2 * BLOCK_WIDTH / 2 + MARGIN));
    select_rect.render();
    ogl::texture* text_texture = nullptr;
    hovered = nullptr;
    for(int i = -1; i < 20; i++)
        updateStackTexture(i);
    for(int i = 0; i < (player_inventory.open ? 20 : 10); i++) {
        if(swl::colliding(inventory_slots[i].getRect(), {(short)swl::mouse_x, (short)swl::mouse_y, 0, 0}) && player_inventory.open) {
            hovered = &player_inventory.inventory[i];
            inventory_slots[i].setColor(70, 70, 70);
            if(player_inventory.inventory[i].item_id != itemEngine::NOTHING) {
                text_texture = &player_inventory.inventory[i].getUniqueItem().text_texture;
                text_texture->setX(swl::mouse_x + 20);
                text_texture->setY(swl::mouse_y + 20);
                under_text_rect.setHeight(text_texture->getHeight() + 2 * MARGIN);
                under_text_rect.setWidth(text_texture->getWidth() + 2 * MARGIN);
                under_text_rect.setX(swl::mouse_x + 20 - MARGIN);
                under_text_rect.setY(swl::mouse_y + 20 - MARGIN);
            }
        }
        else
            inventory_slots[i].setColor(100, 100, 100);
        inventory_slots[i].render();
        renderItem(&player_inventory.inventory[i], inventory_slots[i].getX() + MARGIN / 2, inventory_slots[i].getY() + MARGIN / 2, i);
    }
    if(text_texture) {
        under_text_rect.render();
        text_texture->render();
    }
    renderItem(player_inventory.getMouseItem(), swl::mouse_x, swl::mouse_y, -1);
}

void playerHandler::doPhysics() {
    velocity_y = touchingGround() && velocity_y >= 0 ? short(0) : short(velocity_y + gameLoop::frame_length / 4);
}

void playerHandler::selectSlot(char slot) {
    player_inventory.selected_slot = slot;
    packets::packet packet(packets::HOTBAR_SELECTION);
    packet << slot;
    networking::sendPacket(packet);
}


PACKET_LISTENER(packets::INVENTORY_CHANGE)
    char pos = packet.getChar();
    playerHandler::player_inventory.inventory[(int)pos].setStack(packet.getUShort());
    playerHandler::player_inventory.inventory[(int)pos].item_id = (itemEngine::itemType)packet.getUChar();
PACKET_LISTENER_END

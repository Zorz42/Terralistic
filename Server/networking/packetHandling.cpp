#include "serverNetworking.hpp"
#include "print.hpp"

void ServerNetworkingManager::onPacket(sf::Packet &packet, PacketType packet_type, Connection &conn) {
    ServerPlayer* curr_player = conn.player;
    switch (packet_type) {
        case PacketType::STARTED_BREAKING: {
            unsigned short x, y;
            packet >> x >> y;
            curr_player->breaking_x = x;
            curr_player->breaking_y = y;
            curr_player->breaking = true;
            break;
        }

        case PacketType::STOPPED_BREAKING: {
            curr_player->breaking = false;
            break;
        }

        case PacketType::RIGHT_CLICK: {
            unsigned short x, y;
            packet >> x >> y;
            players->rightClickEvent(blocks->getBlock(x, y), curr_player);
            break;
        }

        case PacketType::VIEW_SIZE_CHANGE: {
            packet >> curr_player->sight_width >> curr_player->sight_height;
            break;
        }

        case PacketType::PLAYER_MOVEMENT: {
            packet >> curr_player->x >> curr_player->y >> curr_player->flipped;
            
            sf::Packet movement_packet;
            movement_packet << PacketType::PLAYER_MOVEMENT << curr_player->x << curr_player->y << curr_player->flipped << curr_player->id;
            sendToEveryone(movement_packet, &conn);
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            unsigned char pos;
            packet >> pos;
            curr_player->inventory.swapWithMouseItem(&curr_player->inventory.inventory_arr[pos]);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            packet >> curr_player->inventory.selected_slot;
            break;
        }

        case PacketType::CHAT: {
            std::string message;
            packet >> message;
            std::string chat_format = (curr_player->name == "_" ? "Protagonist" : curr_player->name) + ": " + message;
            print::info(chat_format);
            
            sf::Packet chat_packet;
            chat_packet << PacketType::CHAT << chat_format;
            sendToEveryone(chat_packet);
            break;
        }
            
        case PacketType::VIEW_POS_CHANGE: {
            packet >> curr_player->sight_x >> curr_player->sight_y;
            break;
        }
            
        case PacketType::CRAFT: {
            unsigned char craft_index;
            packet >> craft_index;
            const Recipe* recipe_crafted = curr_player->inventory.getAvailableRecipes()[(int)craft_index];
            curr_player->inventory.addItem(recipe_crafted->result.type, recipe_crafted->result.stack);
            for(const ItemStack& ingredient : recipe_crafted->ingredients)
                curr_player->inventory.removeItem(ingredient.type, ingredient.stack);
        }

        default:;
    }
}

void ServerNetworkingManager::onEvent(ServerBlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.type;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ServerBlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS_CHANGE << event.block.getX() << event.block.getY() << event.break_stage;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ServerLiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.liquid_type << event.liquid_level;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ServerItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << event.x << event.y << event.id << (unsigned char)event.item_id;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ServerItemDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_DELETION << (short)event.item_to_delete.getId();
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ServerItemMovementEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_MOVEMENT << event.moved_item.getX() <<  event.moved_item.getY() << event.moved_item.getId();
    sendToEveryone(packet);
}

void ServerNetworkingManager::sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack) {
    sf::Packet packet;
    packet << PacketType::INVENTORY_CHANGE << stack << (unsigned char)type << item.getPosInInventory();
    connection.send(packet);
}

void ServerNetworkingManager::onEvent(ServerInventoryItemStackChangeEvent& event) {
    for(Connection& connection : connections)
        if(connection.player && &connection.player->inventory == event.item.getInventory()) {
            sendInventoryItemPacket(connection, event.item, event.item.getType(), event.stack);
            break;
        }
}

void ServerNetworkingManager::onEvent(ServerInventoryItemTypeChangeEvent& event) {
    for(Connection& connection : connections)
        if(connection.player && &connection.player->inventory == event.item.getInventory()) {
            sendInventoryItemPacket(connection, event.item, event.type, event.item.getStack());
            break;
        }
}

void ServerNetworkingManager::onEvent(RecipeAvailabilityChangeEvent& event) {
    for(Connection& connection : connections)
        if(connection.player && &connection.player->inventory == event.inventory) {
            sf::Packet packet;
            packet << PacketType::RECIPE_AVAILABILTY_CHANGE << (unsigned short)event.inventory->getAvailableRecipes().size();
            for(const Recipe* recipe : event.inventory->getAvailableRecipes())
                packet << getRecipeIndex(recipe);
            connection.send(packet);
            break;
        }
}

void ServerNetworkingManager::onEvent(ServerLightChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIGHT_CHANGE << event.block.getX() << event.block.getY() << event.block.getLightLevel();
    for(Connection& connection : connections)
        connection.send(packet);
}

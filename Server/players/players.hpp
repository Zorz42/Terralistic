//
//  players.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#ifndef players_hpp
#define players_hpp

#define INVENTORY_SIZE 20

#include "items.hpp"
#include "packetType.hpp"
#include <SFML/Network.hpp>

class inventory;
class player;

class inventoryItem {
    unsigned short stack;
    inventory* holder;
    ItemType item_id;
public:
    inventoryItem() : holder(nullptr), item_id(ItemType::NOTHING), stack(0) {}
    explicit inventoryItem(inventory* holder) : holder(holder), item_id(ItemType::NOTHING), stack(0) {}

    inline ItemType getId() { return item_id; }
    void setId(ItemType id);
    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    void sendPacket();
};

class inventory {
    friend inventoryItem;
    inventoryItem mouse_item;
    player* owner;
public:
    explicit inventory(player* owner);
    inventoryItem inventory_arr[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    inventoryItem* getSelectedSlot();
    void swapWithMouseItem(inventoryItem* item);
};

class player {
public:
    explicit player(unsigned short id) : id(id), player_inventory(this) {}
    std::string name;
    
    std::string ip;
    bool disconnected = false, registered = false;
    
    
    const unsigned short id;
    bool flipped = false;
    int x = 0, y = 0;
    unsigned short sight_width = 0, sight_height = 0;
    inventory player_inventory;
    
    bool breaking = false;
    unsigned short breaking_x{}, breaking_y{};
    
    sf::TcpSocket* socket;
};

class players;

struct blockEvents {
    void (*onBreak)(blocks*, players*, block*) = nullptr;
    void (*onRightClick)(block*, player*) = nullptr;
    void (*onLeftClick)(block*, player*) = nullptr;
};

class ServerPacketEvent : public Event<ServerPacketEvent> {
public:
    ServerPacketEvent(sf::Packet& packet, player& sender) : packet(packet), sender(sender) {}
    sf::Packet& packet;
    player& sender;
};

class players : EventListener<ServerPacketEvent> {
    items* parent_items;
    blocks* parent_blocks;
    
    std::vector<sf::TcpSocket*> pending_connections;
    std::vector<player*> all_players;
    std::vector<player*> online_players;
    
    void onEvent(ServerPacketEvent& event) override;
    
    void leftClickEvent(block this_block, connection& connection, unsigned short tick_length);
    void rightClickEvent(block this_block, player* peer);
    
    blockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    sf::TcpListener listener;
public:
    players(blocks* parent_blocks_, items* parent_items_);
    
    inline const std::vector<player*>& getAllPlayers() { return all_players; }
    inline const std::vector<player*>& getOnlinePlayers() { return online_players; }
    
    player* getPlayerByConnection(connection* conn);
    player* getPlayerByName(const std::string& name);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocks();
    void lookForItems();
    
    void saveTo(std::string path);
    void loadFrom(std::string path);
    
    void breakBlock(block* this_block);
    
    void openSocket(unsigned short port);
    void closeSocket();
    void sendToEveryone(sf::Packet& packet, player* exclusion=nullptr);
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    
    ~players();
};

#endif /* players_hpp */

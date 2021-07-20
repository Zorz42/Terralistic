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
#include "events.hpp"

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
    void setIdWithoutProcessing(ItemType id);
    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    void setStackWithoutProcessing(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    unsigned char getPosInInventory();
    void syncWithClient();
    inline inventory& getHolderInventory() { return *holder; } 
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
    unsigned char selected_slot = 0;
    inventoryItem* getSelectedSlot();
    void swapWithMouseItem(inventoryItem* item);
    inline player& getOwner() { return *owner; }
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
    void (*onBreak)(Blocks*, players*, Block*) = nullptr;
    void (*onRightClick)(Block*, player*) = nullptr;
    void (*onLeftClick)(Block*, player*) = nullptr;
};

class ServerPacketEvent : public Event<ServerPacketEvent> {
public:
    ServerPacketEvent(sf::Packet& packet, PacketType packet_type, player& sender) : packet(packet), packet_type(packet_type), sender(sender) {}
    sf::Packet& packet;
    PacketType packet_type;
    player& sender;
};

class ServerInventoryItemTypeChangeEvent : public Event<ServerInventoryItemTypeChangeEvent> {
public:
    ServerInventoryItemTypeChangeEvent(inventoryItem& item, ItemType type) : item(item), type(type) {}
    inventoryItem& item;
    ItemType type;
};

class ServerInventoryItemStackChangeEvent : public Event<ServerInventoryItemStackChangeEvent> {
public:
    ServerInventoryItemStackChangeEvent(inventoryItem& item, unsigned short stack) : item(item), stack(stack) {}
    inventoryItem& item;
    unsigned short stack;
};

class players : EventListener<ServerPacketEvent>, EventListener<ServerBlockChangeEvent>, EventListener<ServerBlockBreakStageChangeEvent>, EventListener<ServerLiquidChangeEvent>, EventListener<ServerItemCreationEvent>, EventListener<ServerItemDeletionEvent>, EventListener<ServerItemMovementEvent>, EventListener<ServerInventoryItemStackChangeEvent>, EventListener<ServerInventoryItemTypeChangeEvent> {
    items* parent_items;
    Blocks* parent_blocks;
    
    std::vector<sf::TcpSocket*> pending_connections;
    std::vector<player*> all_players;
    std::vector<player*> online_players;
    
    void onEvent(ServerPacketEvent& event) override;
    void onEvent(ServerBlockChangeEvent& event) override;
    void onEvent(ServerBlockBreakStageChangeEvent& event) override;
    void onEvent(ServerLiquidChangeEvent& event) override;
    void onEvent(ServerItemCreationEvent& event) override;
    void onEvent(ServerItemDeletionEvent& event) override;
    void onEvent(ServerItemMovementEvent& event) override;
    void onEvent(ServerInventoryItemStackChangeEvent& event) override;
    void onEvent(ServerInventoryItemTypeChangeEvent& event) override;
    
    void leftClickEvent(Block this_block, player* peer, unsigned short tick_length);
    void rightClickEvent(Block this_block, player* peer);
    
    blockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    sf::TcpListener listener;
public:
    players(Blocks* parent_blocks_, items* parent_items_);
    
    inline const std::vector<player*>& getAllPlayers() { return all_players; }
    inline const std::vector<player*>& getOnlinePlayers() { return online_players; }
    
    player* getPlayerByName(const std::string& name);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocks();
    void lookForItems();
    
    void saveTo(std::string path);
    void loadFrom(std::string path);
    
    void breakBlock(Block* this_block);
    
    void openSocket(unsigned short port);
    void closeSocket();
    void sendToEveryone(sf::Packet& packet, player* exclusion=nullptr);
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    
    void sendInventoryItemPacket(inventoryItem& item, ItemType type, unsigned short stack);
    
    bool accept_itself = false;
    
    ~players();
};

#endif /* players_hpp */

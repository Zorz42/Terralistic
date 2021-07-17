//
//  clientMap.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#ifndef clientMap_hpp
#define clientMap_hpp

#include <vector>
#include <string>

#include "graphics.hpp"

#include "clientNetworking.hpp"
#include "properties.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

class map : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
public:
    enum class chunkState {unloaded, pending_load, loaded};
protected:
    struct blockData {
        explicit blockData(BlockType block_id=BlockType::AIR, LiquidType liquid_id=LiquidType::EMPTY) : block_id(block_id), liquid_id(liquid_id) {}

        BlockType block_id;
        LiquidType liquid_id;
        unsigned char light_level = 0, break_stage = 0, orientation = 0, liquid_level = 0;
        bool update = true;

        [[nodiscard]] const BlockInfo& getUniqueBlock() const;
        [[nodiscard]] const LiquidInfo& getUniqueLiquid() const;
    };

    struct chunkData {
        chunkState state = chunkState::unloaded;
        bool update = true;
        gfx::Image texture;
    };

    void renderBlocks();
    void renderItems();

    void render() override;
    void onEvent(ClientPacketEvent& event) override;
    void init() override;

    networkingManager* networking_manager;

public:
    class chunk {
        chunkData* chunk_data;
        unsigned short x, y;
        map* parent_map;

    public:
        chunk(unsigned short x, unsigned short y, chunkData* chunk_data, map* parent_map) : x(x), y(y), chunk_data(chunk_data), parent_map(parent_map) {}

        inline chunkState getState() { return chunk_data->state; };
        inline void setState(chunkState state) { chunk_data->state = state; }
        inline bool hasToUpdate() { return chunk_data->update; }
        inline void scheduleUpdate() { chunk_data->update = true; }

        void createTexture();
        void updateTexture();
        void draw();
    };

    class block {
        friend chunk;
        blockData* block_data;
        unsigned short x, y;
        map* parent_map;

        void scheduleTextureUpdate();
        void updateOrientation();
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, map* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
        void setType(BlockType block_id, LiquidType liquid_id);
        void setLightLevel(unsigned char level);
        void setBreakStage(unsigned char stage);
        void draw();
        void update();

        inline bool isGhost() { return block_data->getUniqueBlock().ghost; }
        inline unsigned char getLightLevel() { return block_data->light_level; }
        inline unsigned char getBreakStage() { return block_data->break_stage; }
        inline BlockType getType() { return block_data->block_id; }
        inline LiquidType getLiquidType() { return block_data->liquid_id; }
        inline void setLiquidLevel(unsigned char level) { block_data->liquid_level = level; }
        inline unsigned char getLiquidLevel() { return block_data->liquid_level; }
        inline float getSpeedMultiplier() { return block_data->getUniqueLiquid().speed_multiplier; }
    };

    class item {
        [[nodiscard]] const ItemInfo& getUniqueItem() const;
        unsigned short id;
        ItemType item_type;
    public:
        item(ItemType item_type, int x, int y, unsigned short id) : x(x * 100), y(y * 100), id(id), item_type(item_type) {}
        int x, y;
        [[nodiscard]] unsigned short getId() const { return id; }
        [[nodiscard]] ItemType getType() const { return item_type; }
    };

protected:
    unsigned short width{}, height{};
    chunkData *chunks = nullptr;
    blockData *blocks = nullptr;

    std::vector<item> items;

    unsigned short chunks_pending = 0;
    std::string kick_message;
    bool kicked = false;

    gfx::Image background_image;

public:
    explicit map(networkingManager* manager) : networking_manager(manager) {}
    int view_x{}, view_y{};

    chunk getChunk(unsigned short x, unsigned short y);
    block getBlock(unsigned short x, unsigned short y);

    [[nodiscard]] inline unsigned short getWorldWidth() const { return width; }
    [[nodiscard]] inline unsigned short getWorldHeight() const { return height; }

    void createWorld(unsigned short map_width, unsigned short map_height);

    item* getItemById(unsigned short id);

    ~map() override;
};

#endif /* clientMap_hpp */

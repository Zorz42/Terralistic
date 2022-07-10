#pragma once
#include "blocks.hpp"
#include "serverNetworking.hpp"
#include "worldSaver.hpp"
#include "random"

class ServerPlayerData;
class Entities;


class ServerBlocks : public ServerModule, public Blocks, EventListener<ServerConnectionWelcomeEvent>, EventListener<BlockUpdateEvent>, EventListener<BlockChangeEvent>, EventListener<BlockStartedBreakingEvent>, EventListener<BlockStoppedBreakingEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent> {
    ServerNetworking* networking;
    WorldSaver* world_saver;
    std::mt19937  server_blocks_mt;
    Entities* entities = nullptr;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockUpdateEvent& event) override;
    void onEvent(BlockStartedBreakingEvent& event) override;
    void onEvent(BlockStoppedBreakingEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    void syncBlockData(int x, int y);
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerBlocks(ServerNetworking* networking, WorldSaver* world_saver) : networking(networking), world_saver(world_saver) {}
    
    void updateBlock(int x, int y);
    void setPlayers(Entities* all_entities) { entities = all_entities; };


    EventSender<BlockRandomTickEvent> block_random_tick_event;
};

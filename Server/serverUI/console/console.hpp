#pragma once
#include "launcherModule.hpp"

/*class ChatLine {
public:
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
};

class Chat : public ServerModule, EventListener<ServerPacketEvent> {
    gfx::TextInput chat_box;
    ServerNetworking* networking;
    ServerPlayers* players;
    std::vector<ChatLine*> chat_lines;
    std::vector<std::string> saved_lines = {""};
    int selected_saved_line = 0;

    float chat_width = 100;

    void init() override;
    void update(float frame_length) override;
    //bool onKeyDown(gfx::Key key) override;
    void stop() override;

    void onEvent(ServerPacketEvent& event) override;
public:
    Chat(ServerNetworking* networking, ServerPlayers* players) : ServerModule(), networking(networking), players(players) {}
};
*/




class Console : LauncherModule{
    gfx::TextInput input_box;
    std::vector<std::string> saved_lines = {""};
    int selected_saved_line = 0;
public:
    void update(float frame_length) override;
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    Console(float x_, float y_, float w_, float h_);
};
#pragma once
#include "clientNetworking.hpp"
#include "clientPlayers.hpp"

class ChatLine {
public:
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
    gfx::Timer timer;
};

class Chat : public ClientModule, EventListener<ClientPacketEvent> {
    gfx::TextInput chat_box;
    ClientNetworking* networking;
    ClientPlayers* players;
    std::vector<ChatLine*> chat_lines;
    gfx::Timer timer;
    int timer_counter = 0;
    std::vector<std::string> saved_lines = {""};
    int selected_saved_line = 0;
    
    float chat_width = 100;
    
    void init() override;
    void update(float frame_length) override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;

    void onEvent(ClientPacketEvent& event) override;
public:
    Chat(ClientNetworking* networking, ClientPlayers* players) : ClientModule("Chat"), networking(networking), players(players) {}
};

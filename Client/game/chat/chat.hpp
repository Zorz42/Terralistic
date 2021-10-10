#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"

struct ChatLine {
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
    unsigned int time_created{};
};

class Chat : public ClientModule, EventListener<ClientPacketEvent> {
    gfx::TextInput chat_box;
    NetworkingManager* manager;
    std::vector<ChatLine*> chat_lines;
    
    void init() override;
    void update(float frame_length) override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;

    void onEvent(ClientPacketEvent& event) override;
public:
    Chat(NetworkingManager* manager) : manager(manager) {}
};

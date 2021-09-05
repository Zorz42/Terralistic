#ifndef chat_hpp
#define chat_hpp

#include "graphics.hpp"
#include "clientNetworking.hpp"

struct ChatLine {
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
    unsigned int time_created{};
};

class Chat : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    gfx::TextInput chat_box;
    NetworkingManager* manager;
    std::vector<ChatLine*> chat_lines;
    
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    void stop() override;

    void onEvent(ClientPacketEvent& event) override;
public:
    explicit Chat(NetworkingManager* manager) : manager(manager) {}
};

#endif

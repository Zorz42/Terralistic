//
//  chat.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/06/2021.
//

#ifndef chat_hpp
#define chat_hpp

#include "graphics.hpp"

#include "clientNetworking.hpp"

class chat : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    struct chatLine {
        std::string text;
        gfx::Sprite text_sprite;
        int y_to_be{};
        unsigned int time_created{};
    };

    gfx::TextInput chat_box;
    networkingManager* manager;
    std::vector<chatLine*> chat_lines;
public:
    explicit chat(networkingManager* manager) : manager(manager) {}

    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    void stop() override;

    void onEvent(ClientPacketEvent& event) override;
};

#endif /* chat_hpp */

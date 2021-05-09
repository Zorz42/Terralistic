//
//  multiplayerSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef multiplayerSelector_hpp
#define multiplayerSelector_hpp

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif


#endif

struct multiplayerSelector : gfx::scene {
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
    
private:
    gfx::button back_button, join_button;
    gfx::sprite join_server_title;
    gfx::textInput server_ip;
};

#endif /* multiplayerSelector_hpp */

//
//  clickEvents.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#ifndef clickEvents_hpp
#define clickEvents_hpp

#include "playerHandler.hpp"
#include "map.hpp"

namespace clickEvents {

struct clickEvent {
    void (*rightClickEvent)(map::block*, playerHandler::player*) = nullptr;
    void (*leftClickEvent)(map::block*, playerHandler::player*) = nullptr;
};

inline std::vector<clickEvent> click_events;

void init();

}

#endif /* clickEvents_hpp */

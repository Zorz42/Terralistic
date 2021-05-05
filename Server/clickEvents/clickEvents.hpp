//
//  clickEvents.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#ifndef clickEvents_hpp
#define clickEvents_hpp

#include "serverMap.hpp"

namespace clickEvents {

struct clickEvent {
    void (*rightClickEvent)(map::block*, map::player*) = nullptr;
    void (*leftClickEvent)(map::block*, map::player*) = nullptr;
};

inline std::vector<clickEvent> click_events;

void init();

}

#endif /* clickEvents_hpp */

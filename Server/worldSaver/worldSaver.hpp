#pragma once
#include <vector>
#include "serverModule.hpp"

class WorldSaver : public ServerModule {
    void init() override;
    void stop() override;
public:
    void save();
};

#pragma once

class ServerModule {
public:
    virtual void preInit() {}
    virtual void postInit() {}
    virtual void update(float frame_length) {}
    virtual void stop() {}
};

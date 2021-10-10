#pragma once

class ServerModule {
public:
    virtual void init() {}
    virtual void update(float frame_length) {}
    virtual void stop() {}
};

#pragma once

#include "clientBlocks.hpp"

class ParticleInfo {
public:
    virtual void render(int x, int y) const = 0;
    int lifetime = 0;
    bool gravity = false, colliding = false;
};

class Particle {
    const ParticleInfo* info;
    unsigned int spawned_time = gfx::getTicks();
    float x, y;
public:
    Particle(const ParticleInfo* info, float x, float y) : info(info), x(x), y(y) {}
    const ParticleInfo* getInfo();
    
    bool isGrounded(Blocks* blocks);
    bool isColliding(Blocks* blocks, float colliding_x, float colliding_y);
    bool isColliding(Blocks* blocks);
    
    void update(Blocks* blocks, float frame_length);
    
    int getX();
    int getY();
    
    unsigned int getSpawnedTime();
    
    float velocity_x = 0, velocity_y = 0;
};

class WalkParticle : public ParticleInfo {
public:
    WalkParticle() { lifetime = 1000; gravity = true; colliding = true; }
    void render(int x, int y) const override;
};

inline const WalkParticle walk_particle;

class Particles : public ClientModule {
    ClientBlocks* blocks;
    std::vector<Particle> particles;
    
    void update(float frame_length) override;
    void render() override;
public:
    Particles(ClientBlocks* blocks) : blocks(blocks) {}
    
    void spawnParticle(Particle particle);
};

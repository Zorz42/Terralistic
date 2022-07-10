#pragma once
#include "clientBlocks.hpp"
#include "settings.hpp"

class ParticleInfo {
public:
    virtual void render(int x, int y) const = 0;
    int lifetime = 0;
    bool gravity = false, colliding = false;
};

class Particle {
    const ParticleInfo* info;
    gfx::Timer timer;
    float x, y;
public:
    Particle(const ParticleInfo* info, float x, float y) : info(info), x(x), y(y) {}
    const ParticleInfo* getInfo();
    
    bool isGrounded(Blocks* blocks) const;
    static bool isColliding(Blocks* blocks, float colliding_x, float colliding_y);
    bool isColliding(Blocks* blocks) const;
    
    void update(Blocks* blocks, float frame_length);
    
    int getX() const;
    int getY() const;
    
    int getTimeSpawned();
    
    float velocity_x = 0, velocity_y = 0;
};

class WalkParticle : public ParticleInfo {
public:
    WalkParticle() { lifetime = 1000; gravity = true; colliding = true; }
    void render(int x, int y) const override;
};

inline const WalkParticle walk_particle;

class Particles : public ClientModule {
    Settings* settings;
    ClientBlocks* blocks;
    Camera* camera;
    std::vector<Particle> particles;
    
    void init() override;
    void stop() override;
    void update(float frame_length) override;
    void updateParallel(float frame_length) override;
    void render() override;
    
    BooleanSetting particle_enable_setting;
public:
    Particles(Settings* settings, ClientBlocks* blocks, Camera* camera) : ClientModule("Particles"), settings(settings), blocks(blocks), camera(camera), particle_enable_setting("Particles", true) {}
    
    void spawnParticle(Particle particle);
};

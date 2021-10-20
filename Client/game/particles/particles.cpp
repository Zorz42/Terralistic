#include "particles.hpp"

const ParticleInfo* Particle::getInfo() {
    return info;
}

int Particle::getX() {
    return x;
}

int Particle::getY() {
    return y;
}

unsigned int Particle::getSpawnedTime() {
    return spawned_time;
}

void Particle::update(float frame_length) {
    x += velocity_x * frame_length / 100;
    y += velocity_y * frame_length / 100;
}

void Particles::update(float frame_length) {
    for(Particle& particle : particles)
        particle.update(frame_length);
}

void Particles::render() {
    for(int i = 0; i < particles.size(); i++) {
        particles[i].getInfo()->render(particles[i].getX() - blocks->view_x + gfx::getWindowWidth() / 2, particles[i].getY() - blocks->view_y + gfx::getWindowHeight() / 2);
        
        if(gfx::getTicks() - particles[i].getSpawnedTime() > particles[i].getInfo()->lifetime)
            particles.erase(particles.begin() + i);
    }
}

void WalkParticle::render(int x, int y) const {
    gfx::RectShape(x - 1, y - 1, 2, 2).render({100, 100, 100});
}

void Particles::spawnParticle(Particle particle) {
    particles.push_back(particle);
}

#include "naturalLight.hpp"

float dayFunction(float a) {
    a -= (int)a;
    
    if(a < 0.25)
        return 1;
    if(a < 0.5)
        return -a * 4 + 2;
    if(a < 0.75)
        return 0;
    return a * 4 - 3;
}

void NaturalLight::init() {
    blocks->block_change_event.addListener(this);
    networking->packet_event.addListener(this);
    debug_menu->registerDebugLine(&day_time_line);
}

void NaturalLight::postInit() {
    lights_arr = new int[blocks->getWidth()];
    for(int x = 0; x < blocks->getWidth(); x++)
        lights_arr[x] = -1;
    natural_light_thread = std::thread(&NaturalLight::naturalLightUpdateLoop, this);
}

void NaturalLight::stop() {
    running = false;
    natural_light_thread.join();
    blocks->block_change_event.removeListener(this);
    networking->packet_event.removeListener(this);
}

void NaturalLight::onEvent(BlockChangeEvent &event) {
    removeNaturalLight(event.x);
    updateLight(event.x);
}

void NaturalLight::onEvent(ClientPacketEvent &event) {
    if(event.packet_type == ServerPacketType::TIME) {
        int server_time;
        event.packet >> server_time;
        server_timer.set(server_time);
    }
}

void NaturalLight::naturalLightUpdateLoop() {
#ifdef __APPLE__
    pthread_setname_np("Natural Light Update");
#endif

#ifdef __linux__
    pthread_setname_np(pthread_self(), "Natural Light Update");
#endif
    
    while(running) {
        day_time_line.text = std::string("Time: ") + std::to_string(server_timer.getTimeElapsed() / 1000);
        light_should_be = dayFunction((float)getTime() / 1000 / SECONDS_PER_DAY) * MAX_LIGHT;

        for(int x = blocks->getBlocksExtendedViewBeginX(); x <= blocks->getBlocksExtendedViewEndX(); x++)
            updateLight(x);
        
        gfx::sleep(20);
    }
}

void NaturalLight::setNaturalLight(int x, int power) {
    if(x < 0 || x >= blocks->getWidth())
        throw Exception("Natural light x out of range");
    if(lights_arr == nullptr)
        throw Exception("lights_arr is null");

    if(lights_arr[x] != power) {
        lights_arr[x] = power;
        for(int y = 0; y < blocks->getHeight(); y++) {
            lights->setLightSource(x, y, LightColor(0, 0, 0));
            lights->updateLightEmitter(x, y);
        }
        
        for(int y = 0; y < blocks->getHeight() && blocks->getBlockType(x, y)->transparent; y++) {
            LightColor source_color = lights->getLightSourceColor(x, y);
            source_color.r = std::max(source_color.r, power);
            source_color.g = std::max(source_color.g, power);
            source_color.b = std::max(source_color.b, power);
            lights->setLightSource(x, y, source_color);
        }
    }
}

void NaturalLight::removeNaturalLight(int x) {
    if(x < 0 || x >= blocks->getWidth())
        throw Exception("Natural light x out of range");
    if(lights_arr == nullptr)
        throw Exception("lights_arr is null");
    lights_arr[x] = 0;
    for(int y = 0; y < blocks->getHeight(); y++)
        lights->setLightSource(x, y, LightColor(0, 0, 0));
}

long long NaturalLight::getTime() const {
    return server_timer.getTimeElapsed();
}

void NaturalLight::updateLight(int x) {
    setNaturalLight(x, light_should_be);
}

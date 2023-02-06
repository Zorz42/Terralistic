#pragma once
#include <thread>
#include <utility>
#include "menuBack.hpp"

enum class UpdateState {NEUTRAL, CHECKING, DOWNLOADING, APPLYING, FINISHED};

class UpdateChecker : public gfx::Scene {
    void checkForUpdates();
    UpdateState update_state = UpdateState::NEUTRAL;
    std::thread update_thread;
    std::string exec_path;
    gfx::Sprite text;
    bool has_updated = false;
    void init() override;
    void render() override;
    MenuBack* menu_back;
public:
    UpdateChecker(MenuBack* menu_back, std::string exec_path) : gfx::Scene("UpdateChecker"), menu_back(menu_back), exec_path(std::move(exec_path)) {}
};

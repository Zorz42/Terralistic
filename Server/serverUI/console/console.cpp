#include "console.hpp"
#include <set>
#include <platform_folders.h>
#include <fstream>
#include <iomanip>

static const std::set<char> allowed_chars = {'!', ':', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '"', '|', '~', '<', '>', '?', '-', '=', ',', '.', '/', '[', ']', ';', '\'', '\\', '`', ' '};

Console::Console(std::string resource_path): LauncherModule("console", std::move(resource_path)) {
    /*min_width = 300;
    min_height = 90;
    texture.createBlankImage(width, height);

    auto t = std::time(nullptr);
    auto tm = *localtime(&t);
    std::stringstream timestamped_text;
    timestamped_text << std::put_time(&tm, "log_@%Y.%m.%d_%H:%M:%S.txt");
    log_file_name = timestamped_text.str();*/
}

void Console::init() {
   /* input_box.scale = 2;
    input_box.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || allowed_chars.find(c) != allowed_chars.end())
            return c;
        return '\0';
    };

    input_box.def_color.a = 255;
    input_box.setBlurIntensity(1);
    input_box.setBorderColor(BORDER_COLOR);
    input_box.width = 100;
    input_box.setPassthroughKeys({gfx::Key::ARROW_UP, gfx::Key::ARROW_DOWN});
    text_inputs = {&input_box};
    server->getPrint()->print_event.addListener(this);*/
}



void Console::render() {
    /*if(!enabled)
        return;

    if(width != texture.getTextureWidth() || height != texture.getTextureHeight()) {
        texture.createBlankImage(width, height);

        for(auto & chat_line : chat_lines) {
            chat_line->text_sprite.x = input_box.x;
            chat_line->y_to_be = (int) input_box.y - input_box.getHeight();

            for (auto &i2: chat_lines)
                if (i2 != chat_line)
                    i2->y_to_be -= chat_line->text_sprite.getHeight();
        }

    }
    input_box.width = width / 2 - 15;
    input_box.x = 10;
    input_box.y = (float)height - 10 - (float)input_box.getHeight();

    for(auto & chat_line : chat_lines)
        chat_line->text_sprite.y += ((float)chat_line->y_to_be - chat_line->text_sprite.y) / 2;

    texture.setRenderTarget();

    gfx::RectShape(0, 0, width, height).render(DARK_GREY);//can be removed once setMinimumWindoeSize works
    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);
    input_box.render(getMouseX(), getMouseY());//you are not able to click directly onto the text input if this module is not in the upper right position. will be fixed with the implementation of UI containers


    for(auto & chat_line : chat_lines) {
        if(!chat_line->text.empty()) {
            chat_line->text_sprite.scale = 1.5;
            chat_line->text_sprite.loadFromText(chat_line->text);
            chat_line->text_sprite.y = input_box.y;
            chat_line->text_sprite.x = input_box.x;
            chat_line->y_to_be = (int)input_box.y - input_box.getHeight();
            chat_line->text.clear();
            chat_line->text.shrink_to_fit();

            for(auto & i2 : chat_lines)
                if(i2 != chat_line)
                    i2->y_to_be -= chat_line->text_sprite.getHeight();
        }

        chat_line->text_sprite.render();
    }

    gfx::resetRenderTarget();*/
}

bool Console::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ENTER && input_box.active) {
        if(!input_box.getText().empty()) {
            server->getPrint()->info("[Server] " + input_box.getText());
            if(input_box.getText().substr(0, 2) == "//"){//when more // commands get added code inside this block will be moved to its own sub-function
                std::string command = input_box.getText();
                command.erase(0, 2);
                if(command.substr(0, 13) == "module_config"){
                    module_manager->moduleConfig(command);
                }
            }else {
                std::string command = input_box.getText();
                if(command.at(0) != '/')
                    command.insert(command.begin(), '/');
                ServerChatEvent event(nullptr, command);
                server->getChat()->chat_event.call(event);
            }
            saved_lines.insert(saved_lines.begin() + 1, input_box.getText());
            selected_saved_line = 0;
            if (saved_lines.size() > 20)
                saved_lines.erase(saved_lines.end());
            input_box.setText("");
        }
        input_box.active = false;
        return true;
    } else if(key == gfx::Key::ESCAPE && input_box.active) {
        input_box.setText("");
        selected_saved_line = 0;
        input_box.active = false;
        return true;
    }
    if(key == gfx::Key::ARROW_UP && input_box.active) {
        if(selected_saved_line < saved_lines.size() - 1) {
            selected_saved_line++;
            input_box.setText(saved_lines[selected_saved_line]);
            input_box.setCursor(input_box.getText().size());
        }
        return true;
    } else if(key == gfx::Key::ARROW_DOWN && input_box.active) {
        if(selected_saved_line > 0){
            selected_saved_line--;
            input_box.setText(saved_lines[selected_saved_line]);
            input_box.setCursor(input_box.getText().size());
        }
        return true;
    }
    return false;
}

void Console::onEvent(PrintEvent &event) {
    while(chat_lines.size() > 100)
        chat_lines.erase(chat_lines.begin());

    std::string curr_line, whole_message;
    whole_message = event.message;
    if(!whole_message.ends_with("\n"))
        whole_message.push_back('\n');
    saveToLog(whole_message);
    while(!whole_message.empty()) {
        curr_line.push_back(whole_message[0]);
        whole_message.erase(whole_message.begin());
        if(whole_message[0] == '\n') {
            auto* new_line = new ChatLine;
            new_line->text = curr_line;
            chat_lines.push_back(new_line);
            whole_message.erase(whole_message.begin());
            curr_line = "";

            auto line = chat_lines[chat_lines.size() - 1];
            if(event.type == MessageType::WARNING)
                line->text_sprite.setColor({255, 255, 0});
            else if(event.type == MessageType::ERROR)
                line->text_sprite.setColor({255, 0, 0});
        }
    }
}

void Console::saveToLog(const std::string& line) {
    std::ofstream file;
    file.open(sago::getDataHome() + "/Terralistic-Server/serverLogFiles/" + log_file_name, std::ios::out | std::ios::app);
    file << line;
    file.close();
}


void Console::stop() {
    //server->getPrint()->print_event.removeListener(this);
}

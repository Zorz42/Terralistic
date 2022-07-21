#include "clientPlayers.hpp"
#include "readOpa.hpp"

ClientPlayer::ClientPlayer(const std::string& name, int x, int y, int id) : Player(x, y, name, id) {
    friction = false;
}

void ClientPlayers::loadPlayerTexture() {
    std::vector<unsigned char> skin_template, player_texture_vector, skin;
    loadOpaSkinTemplate(skin_template, resource_pack->getFile("/misc/skin_template.opa"));
    loadOpaSkinTemplate(skin, resource_pack->getFile("/misc/skin.opa"));

    int width = *(int*)&skin_template[0];
    int height = *(int*)&skin_template[sizeof(int)];
    skin_template.erase(skin_template.begin(), skin_template.begin() + sizeof(int) * 2);
    skin.erase(skin.begin(), skin.begin() + sizeof(int) * 2);
    player_texture_vector.resize(skin_template.size());

    for(int i = 0; i < skin_template.size(); i += 4)
        if(skin_template[i + 3] == 0){
            player_texture_vector[i] = 0;
            player_texture_vector[i + 1] = 0;
            player_texture_vector[i + 2] = 0;
            player_texture_vector[i + 3] = 0;
        }else{
            int x = skin_template[i + 2] / 8;
            int y = 31 - skin_template[i + 1] / 8;
            int pixel = (y * 32 + x) * 4;//x and y may need to be reversed
            unsigned char r = skin[pixel];
            unsigned char g = skin[pixel + 1];
            unsigned char b = skin[pixel + 2];
            unsigned char a = skin[pixel + 3];
            if(pixel * 2 < 4096)//size of the skin array
                a = 255;
            player_texture_vector[i] = r;
            player_texture_vector[i + 1] = g;
            player_texture_vector[i + 2] = b;
            player_texture_vector[i + 3] = a;
        }

    //player_texture.loadFromData(&player_texture_vector[0], width, height);
    //main_player->player_texture.loadFromData(&player_texture_vector[0], width, height);
    loadOpa(main_player->player_texture, resource_pack->getFile("/misc/player.opa"));
    main_player->has_created_texture = true;
}

void ClientPlayers::render() {
    for(auto i : entities->getEntities())
        if(i->type == EntityType::PLAYER) {
            ClientPlayer* player = (ClientPlayer*)i;
            render(*player);
            if(player->has_moved_x && player->isTouchingGround(blocks) && rand() % 100 < abs(player->getVelocityX()) * 2.5) {
                Particle particle(&walk_particle, player->getX() + int(player->getWidth() / 2), player->getY() + player->getHeight());
                particle.velocity_x = -player->getVelocityX() / 4 + rand() % int(std::abs(player->getVelocityX())) - std::abs(player->getVelocityX()) / 2;
                particle.velocity_y = -rand() % int(std::abs(player->getVelocityX()) / 3 * 2) - std::abs(player->getVelocityX()) / 2;
                particles->spawnParticle(particle);
                
            }
        }
}

#define HEADER_MARGIN 4
#define HEADER_PADDING 2

void ClientPlayers::render(ClientPlayer& player_to_draw) {
    if(player_to_draw.isTouchingGround(blocks) && player_to_draw.getVelocityY() == 0)
        player_to_draw.has_jumped = false;
    
    if(player_to_draw.getVelocityX())
        player_to_draw.flipped = player_to_draw.getVelocityX() < 0;
    
    if(player_to_draw.moving_type == MovingType::STANDING)
        player_to_draw.started_moving = 0;
    
    if(player_to_draw.moving_type == MovingType::SNEAKING)
        player_to_draw.texture_frame = 10;
    else if(player_to_draw.has_jumped)
        player_to_draw.texture_frame = 0;
    else if(player_to_draw.moving_type == MovingType::WALKING && player_to_draw.has_moved_x)
        player_to_draw.texture_frame = int(timer.getTimeElapsed() - player_to_draw.started_moving) / 70 % 9 + 1;
    else if(player_to_draw.moving_type == MovingType::SNEAK_WALKING) {
        if(player_to_draw.has_moved_x)
            player_to_draw.texture_frame = int(timer.getTimeElapsed() - player_to_draw.started_moving) / 150 % 6 + 10;
        else
            player_to_draw.texture_frame = 10;
    } else if(player_to_draw.moving_type == MovingType::RUNNING && player_to_draw.has_moved_x)
        player_to_draw.texture_frame = int(timer.getTimeElapsed() - player_to_draw.started_moving) / 80 % 8 + 16;
    else
        player_to_draw.texture_frame = 1;
    
    int player_x = gfx::getWindowWidth() / 2 + player_to_draw.getX() - camera->getX();
    int player_y = gfx::getWindowHeight() / 2 + player_to_draw.getY() - camera->getY();
    /*if(player_to_draw.has_created_texture)
        player_to_draw.player_texture.render(2, player_x - 4, player_y - 8, {player_to_draw.texture_frame * (PLAYER_WIDTH + 4), 0, (PLAYER_WIDTH + 4), (PLAYER_HEIGHT + 4)}, player_to_draw.flipped);
    else
        player_texture.render(2, player_x - 4, player_y - 8, {player_to_draw.texture_frame * (PLAYER_WIDTH + 4), 0, (PLAYER_WIDTH + 4), (PLAYER_HEIGHT + 4)}, player_to_draw.flipped);
    */
    if(player_to_draw.has_created_texture)
        player_to_draw.player_texture.render(2, player_x, player_y);
    else
        player_texture.render(2, player_x, player_y);
    if(&player_to_draw != main_player) {
        if(!player_to_draw.has_created_text) {
            player_to_draw.name_text.loadFromText(player_to_draw.name, WHITE);
            player_to_draw.has_created_text = true;
        }
        int header_x = gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.getX() + PLAYER_WIDTH - camera->getX(),
        header_y = gfx::getWindowHeight() / 2 + player_to_draw.getY() - camera->getY() - player_to_draw.name_text.getTextureHeight() - HEADER_MARGIN;
        gfx::RectShape(header_x - HEADER_PADDING, header_y - HEADER_PADDING, player_to_draw.name_text.getTextureWidth() + 2 * HEADER_PADDING, player_to_draw.name_text.getTextureHeight() + 2 * HEADER_PADDING).render(BLACK);
        player_to_draw.name_text.render(1, header_x, header_y);
    }
}

ClientPlayer* ClientPlayers::getPlayerById(int id) {
    for(auto i : entities->getEntities())
        if(i->type == EntityType::PLAYER && i->id == id)
            return (ClientPlayer*)i;
    throw Exception("Player not found by id");
}

void ClientPlayers::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::PLAYER_JOIN: {
            int x, y;
            int id;
            int moving_type;
            std::string name;
            event.packet >> x >> y >> id >> name >> moving_type;
            ClientPlayer* new_player = new ClientPlayer(name, x, y, id);
            new_player->moving_type = (MovingType)moving_type;
            entities->registerEntity(new_player);
            if(name == username) {
                main_player = new_player;
                main_player->ignore_server_updates = true;
                camera->setX(main_player->getX() + PLAYER_WIDTH);
                camera->setY(main_player->getY() + PLAYER_HEIGHT - 2000);
                camera->jumpToTarget();
                loadPlayerTexture();
            }

            break;
        }
        case ServerPacketType::PLAYER_MOVING_TYPE: {
            int id;
            int moving_type;
            event.packet >> moving_type >> id;
            if(main_player && id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->moving_type = (MovingType)moving_type;
            }
            
            break;
        }
        case ServerPacketType::PLAYER_JUMPED: {
            int id;
            event.packet >> id;
            if(main_player && id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->has_jumped = true;
            }
            
            break;
        }
        case ServerPacketType::ENTITY_POSITION: {
            Packet event_packet = event.packet;
            int id;
            int x, y;
            event_packet >> x >> y >> id;
            
            Entity* entity = entities->getEntityById(id);
            if(entity == main_player) {
                Packet packet;
                packet << ClientPacketType::MAIN_PLAYER_POSITION << main_player->getX() << main_player->getY();
                networking->sendPacket(packet);
            }

            break;
        }
        case ServerPacketType::MAIN_PLAYER_POSITION: {
            int x, y;
            event.packet >> x >> y;
            
            entities->setX(main_player, x);
            entities->setY(main_player, y);
            break;
        }
        case ServerPacketType::ENTITY_DELETION: {
            Packet event_packet = event.packet;
            int id;
            event_packet >> id;

            if(main_player && id == main_player->id)
                main_player = nullptr;
            break;
        }
        default:;
    }
}

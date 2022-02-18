#include "clientPlayers.hpp"

ClientPlayer::ClientPlayer(const std::string& name, int x, int y, int id) : Player(x, y, name, id) {
    name_text.loadFromText(name, WHITE);
    friction = false;
}

void ClientPlayers::render() {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER) {
            ClientPlayer* player = (ClientPlayer*)entities->getEntities()[i];
            render(*player);
            if(player->has_moved_x && player->isTouchingGround(blocks) && rand() % 100 < abs(player->getVelocityX()) * 2.5) {
                Particle particle(&walk_particle, player->getX() + player->getWidth() / 2, player->getY() + player->getHeight());
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
    player_texture.render(2, player_x, player_y, {player_to_draw.texture_frame * PLAYER_WIDTH, 0, PLAYER_WIDTH, PLAYER_HEIGHT}, player_to_draw.flipped);
    if(player_to_draw.name != "_") {
        int header_x = gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.getX() + PLAYER_WIDTH - camera->getX(),
        header_y = gfx::getWindowHeight() / 2 + player_to_draw.getY() - camera->getY() - player_to_draw.name_text.getTextureHeight() - HEADER_MARGIN;
        gfx::RectShape(header_x - HEADER_PADDING, header_y - HEADER_PADDING, player_to_draw.name_text.getTextureWidth() + 2 * HEADER_PADDING, player_to_draw.name_text.getTextureHeight() + 2 * HEADER_PADDING).render(BLACK);
        player_to_draw.name_text.render(1, header_x, header_y);
    }
}

ClientPlayer* ClientPlayers::getPlayerById(int id) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::PLAYER && entities->getEntities()[i]->id == id)
            return (ClientPlayer*)entities->getEntities()[i];
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
            }

            break;
        }
        case ServerPacketType::PLAYER_MOVING_TYPE: {
            int id;
            int moving_type;
            event.packet >> moving_type >> id;
            if(id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->moving_type = (MovingType)moving_type;
            }
            
            break;
        }
        case ServerPacketType::PLAYER_JUMPED: {
            int id;
            event.packet >> id;
            if(id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->has_jumped = true;
            }
            
            break;
        }
        case ServerPacketType::ENTITY_POSITION: {
            sf::Packet event_packet = event.packet;
            int id;
            int x, y;
            event_packet >> x >> y >> id;
            
            Entity* entity = entities->getEntityById(id);
            if(entity == main_player) {
                sf::Packet packet;
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
            int id;
            event.packet >> id;

            if(id == main_player->id)
                main_player = nullptr;
            break;
        }
        default:;
    }
}

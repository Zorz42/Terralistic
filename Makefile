MODE 			?=		DEBUG
SFML_DIR		?=		Deps/SFML
CC				?=		g++
CFLAGS 			?=		-Wall -std=c++17
CXXFLAGS		?= 		-Ithird_party -Ithird_party/properties -Ithird_party/simplexNoise -Ithird_party/events -Ithird_party/graphics -Ithird_party/graphics/scene -Ithird_party/graphics/rect -Ithird_party/graphics/ui -Ithird_party/packetType -Ithird_party/resourcePath -Ithird_party/sago -Ithird_party/configManager -I$(SFML_DIR)/include
LDFLAGS			?=		-lm -lpthread -lstdc++ -L$(SFML_DIR)/lib -lsfml-system -lsfml-graphics -lsfml-audio -lsfml-network -lsfml-window

ifeq ($(MODE),DEBUG)
	CFLAGS 	+= -Og -g3
else
	ifeq ($(MODE),RELEASE)
		CFLAGS += -march=native -mtune=native -Ofast -pipe
		LDFLAGS += -fno-pie -fdata-sections -ffunction-sections -static-libgcc -static-libstdc++ -s -Wl,--as-needed -Wl,--gc-sections
	else
		@echo "Mode not supported"
	endif
endif

all: client server

server: Server/main.cpp $(wildcard Server/src/*.cpp)
	@echo "Compiling Server"
	@$(CC) $(CFLAGS) $(CXXFLAGS) -IServer/include $(wildcard Server/src/*.cpp) Server/main.cpp third_party/properties/properties.cpp third_party/simplexNoise/SimplexNoise.cpp third_party/graphics/scene/scene.cpp third_party/graphics/graphics.cpp third_party/graphics/rect/rect.cpp third_party/graphics/ui/ui.cpp third_party/packetType/packetType.cpp third_party/resourcePath/resourcePath.cpp third_party/sago/sago.cpp third_party/configManager/configManager.cpp $(LDFLAGS) -o Terralistic-Server
	@echo "Compiled Server"

client: Client/main.cpp $(wildcard Client/src/*.cpp) $(wildcard Server/src/*.cpp)
	@echo "Compiling Client"
	@$(CC) $(CFLAGS) $(CXXFLAGS) -IClient/include -IServer/include Client/main.cpp $(wildcard Client/src/*.cpp) third_party/properties/properties.cpp third_party/simplexNoise/SimplexNoise.cpp third_party/graphics/scene/scene.cpp third_party/graphics/graphics.cpp third_party/graphics/rect/rect.cpp third_party/graphics/ui/ui.cpp third_party/packetType/packetType.cpp third_party/resourcePath/resourcePath.cpp third_party/sago/sago.cpp third_party/configManager/configManager.cpp $(wildcard Server/src/*.cpp) $(LDFLAGS) -o Terralistic
	@echo "Compiled Client"


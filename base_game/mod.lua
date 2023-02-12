--[[

This is a Terralistic mod file.
It is used to define the mod's name,
description, version and the behavior
of the mod.

]]--


-- This function returns the mod's name.
function mod_name()
    return "base_game"
end

-- This function returns the mod's description.
function mod_description()
    return "The base game. It contains the basic"..
    "game mechanics and the classic Terralistic"..
    "experience."
end

-- This function returns the mod's version.
function mod_version()
    return "0.1"
end

-- global variables for block IDs
air = 0
dirt_block = 0
stone_block = 0
copper_ore = 0
grass_block = 0

-- global variables for wall IDs
dirt_wall = 0

-- global variables for biome IDs
plains = 0
hills = 0
mountains = 0

-- global variables for item IDs
dirt_item = 0

-- This function is called when the mod is loaded.
function init()
    -- register blocks
    air = terralistic_get_block_id_by_name("air")

    -- DIRT
    block_type = terralistic_new_block_type()
    block_type["name"] = "dirt"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    dirt_block = terralistic_register_block_type(block_type)

    -- STONE
    block_type = terralistic_new_block_type()
    block_type["name"] = "stone_block"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    stone_block = terralistic_register_block_type(block_type)

    -- COPPER ORE
    block_type = terralistic_new_block_type()
    block_type["name"] = "copper_ore"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    copper_ore = terralistic_register_block_type(block_type)

    -- GRASS BLOCK
    block_type = terralistic_new_block_type()
    block_type["name"] = "grass_block"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    grass_block = terralistic_register_block_type(block_type)

    terralistic_connect_blocks(dirt_block, grass_block);

    -- register walls
    wall_type = terralistic_new_wall_type()
    wall_type["name"] = "dirt"
    dirt_wall = terralistic_register_wall_type(wall_type)

    -- register items
    item_type = terralistic_new_item_type()
    item_type["name"] = "dirt"
    item_type["display_name"] = "Dirt Block"
    dirt_item = terralistic_register_item_type(item_type)

    terralistic_set_block_drop(dirt_block, dirt_item, 1)

    terralistic_print("base_game mod loaded.")
end

-- This function is called when the mod is loaded on a server.
function init_server()
    -- register biomes

    -- PLAINS
    biome = terralistic_new_biome()
    biome["min_terrain_height"] = 0
    biome["max_terrain_height"] = 40
    biome["min_width"] = 100
    biome["max_width"] = 300
    biome["base_block"] = dirt_block
    biome["base_wall"] = dirt_wall
    biome["generator_function"] = "generate_plains"
    biome:add_ore(stone_block, -2.0, 3.0);
    biome:add_ore(copper_ore, -0.9, -0.4);
    plains = terralistic_register_biome(biome)

    -- HILLS
    biome = terralistic_new_biome()
    biome["min_terrain_height"] = 10
    biome["max_terrain_height"] = 60
    biome["min_width"] = 100
    biome["max_width"] = 300
    biome["base_block"] = dirt_block
    biome["base_wall"] = dirt_wall
    biome["generator_function"] = "generate_plains"
    biome:add_ore(stone_block, -2.0, 3.0);
    biome:add_ore(copper_ore, -0.9, -0.4);
    hills = terralistic_register_biome(biome)
    terralistic_connect_biomes(plains, hills, 100)

    -- MOUNTAINS
    biome = terralistic_new_biome()
    biome["min_terrain_height"] = 40
    biome["max_terrain_height"] = 120
    biome["min_width"] = 100
    biome["max_width"] = 300
    biome["base_block"] = dirt_block
    biome["base_wall"] = dirt_wall
    biome:add_ore(stone_block, 1.0, 1.0);
    biome:add_ore(copper_ore, -0.42, -0.38);
    mountains = terralistic_register_biome(biome)
    terralistic_connect_biomes(hills, mountains, 100)
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("base_game mod stopped.")
end

-- This function is called every frame.
function update()

end

function generate_plains(terrain, width, height)
    for x = 1, width do
        y = 1
        while y < height and terrain[x][y] == air do
            y = y + 1
        end

        if y < height then
            terrain[x][y] = grass_block
        end
    end
    return terrain
end
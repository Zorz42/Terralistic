--[[

This is a Terralistic mod file.
It is used to define the mod's name,
description, version and the behavior
of the mod.

]]--


-- This function returns the mod's name.
function mod_name()
    return "BaseGame"
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
air = 0 -- air is built-in and the id is always 0
dirt_block = -1
stone_block = -1
copper_ore = -1

-- global variables for biome IDs
plains = -1
hills = -1
mountains = -1

-- This function is called when the mod is loaded.
function init()
    -- register blocks

    -- DIRT
    block_type = terralistic_new_block_type()
    block_type["name"] = "dirt"
    block_type["can_update_states"] = true
    dirt_block = terralistic_register_block_type(block_type)

    -- STONE
    block_type = terralistic_new_block_type()
    block_type["name"] = "stone_block"
    block_type["can_update_states"] = true
    stone_block = terralistic_register_block_type(block_type)

    -- COPPER ORE
    block_type = terralistic_new_block_type()
    block_type["name"] = "copper_ore"
    block_type["can_update_states"] = true
    copper_ore = terralistic_register_block_type(block_type)

    terralistic_print("BaseGame mod loaded.")
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
    biome:add_ore(stone_block, 1.0, 1.0);
    biome:add_ore(copper_ore, -0.42, -0.38);
    mountains = terralistic_register_biome(biome)
    terralistic_connect_biomes(hills, mountains, 100)
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("BaseGame mod stopped.")
end

-- This function is called every frame.
function update()

end
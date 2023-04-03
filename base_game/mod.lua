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
    return "The base game. It contains the basic" ..
            "game mechanics and the classic Terralistic" ..
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
copper_ore_block = 0
grass_block = 0
wood_block = 0
branch_block = 0
leaves_block = 0

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
    copper_ore_block = terralistic_register_block_type(block_type)

    -- GRASS BLOCK
    block_type = terralistic_new_block_type()
    block_type["name"] = "grass_block"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    grass_block = terralistic_register_block_type(block_type)

    terralistic_connect_blocks(dirt_block, grass_block);

    -- WOOD
    block_type = terralistic_new_block_type()
    block_type["name"] = "wood"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    block_type["ghost"] = true
    block_type["transparent"] = true
    wood_block = terralistic_register_block_type(block_type)

    -- BRANCH
    block_type = terralistic_new_block_type()
    block_type["name"] = "branch"
    block_type["break_time"] = 1000
    block_type["ghost"] = true
    block_type["transparent"] = true
    branch_block = terralistic_register_block_type(block_type)

    -- LEAVES
    block_type = terralistic_new_block_type()
    block_type["name"] = "leaves"
    block_type["break_time"] = 1000
    block_type["ghost"] = true
    block_type["transparent"] = true
    leaves_block = terralistic_register_block_type(block_type)

    terralistic_connect_blocks(leaves_block, wood_block);

    -- register walls
    wall_type = terralistic_new_wall_type()
    wall_type["name"] = "dirt"
    dirt_wall = terralistic_register_wall_type(wall_type)

    -- register items
    item_type = terralistic_new_item_type()
    item_type["name"] = "dirt"
    item_type["display_name"] = "Dirt Block"
    item_type["max_stack"] = 99
    item_type["places_block"] = dirt_block
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
    biome:add_ore(copper_ore_block, -0.9, -0.4);
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
    biome:add_ore(copper_ore_block, -0.9, -0.4);
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
    biome:add_ore(copper_ore_block, -0.42, -0.38);
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

function generate_plains(terrain, heights, width, height)
    for x = 1, width do
        if terrain[x][height - heights[x] + 1] == dirt_block then
            terrain[x][height - heights[x] + 1] = grass_block
        end
    end

    -- every 5 - 20 block there is a tree with a height of 7 - 20 blocks
    -- the tree is made of wood blocks, if there is the same height
    -- on the left or right, there is a 50% chance for each to spawn additional
    -- wood block on the left or right

    x = 3
    while x < width - 2 do
        tree_height = math.random(7, 20)
        tree_y = height - heights[x]
        if terrain[x][tree_y + 1] == grass_block then
            for y = tree_y - tree_height, tree_y do
                if y > 0 and y < height then
                    terrain[x][y] = wood_block
                end
            end

            if heights[x] == heights[x + 1] and terrain[x + 1][tree_y + 1] == grass_block and math.random(0, 1) == 0 then
                terrain[x + 1][tree_y] = wood_block
            end

            if heights[x] == heights[x - 1] and terrain[x - 1][tree_y + 1] == grass_block and math.random(0, 1) == 0 then
                terrain[x - 1][tree_y] = wood_block
            end

            leave_y = tree_y - tree_height + math.random(3, 7)
            while leave_y < tree_y do
                if terrain[x - 1][leave_y] == air and terrain[x - 2][leave_y] == air then
                    terrain[x - 1][leave_y] = branch_block
                    terrain[x - 2][leave_y] = leaves_block
                end
                leave_y = leave_y + math.random(3, 10);
            end

            leave_y = tree_y - tree_height + math.random(3, 7)
            while leave_y < tree_y do
                if terrain[x + 1][leave_y] == air and terrain[x + 2][leave_y] == air then
                    terrain[x + 1][leave_y] = branch_block
                    terrain[x + 2][leave_y] = leaves_block
                end
                leave_y = leave_y + math.random(3, 10);
            end
        end

        x = x + math.random(6, 15)
    end


    return terrain
end
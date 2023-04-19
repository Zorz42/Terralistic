--[[

This is a Terralistic mod file.
It is used to define the mod's name,
description, version and the behavior
of the mod.

]]--


MOD_NAME = "base_game"
MOD_DESCRIPTION = "The base game. It contains the basic" ..
            "game mechanics and the classic Terralistic" ..
            "experience."
VERSION = "0.1"

blocks = {}
walls = {}
biomes = {}
items = {}
tools = {}

-- This function is called when the mod is loaded.
function init()
    -- register tools
    tools.axe = terralistic_register_tool("axe")




    -- register blocks
    blocks.air = terralistic_get_block_id_by_name("air")

    -- DIRT
    block_type = terralistic_new_block_type()
    block_type["name"] = "dirt"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    blocks.dirt = terralistic_register_block_type(block_type)

    -- STONE
    block_type = terralistic_new_block_type()
    block_type["name"] = "stone_block"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    blocks.stone_block = terralistic_register_block_type(block_type)

    -- COPPER ORE
    block_type = terralistic_new_block_type()
    block_type["name"] = "copper_ore"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    blocks.copper_ore = terralistic_register_block_type(block_type)

    -- GRASS BLOCK
    block_type = terralistic_new_block_type()
    block_type["name"] = "grass_block"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    blocks.grass_block = terralistic_register_block_type(block_type)

    terralistic_connect_blocks(blocks.dirt, blocks.grass_block);

    -- WOOD
    block_type = terralistic_new_block_type()
    block_type["name"] = "wood"
    block_type["can_update_states"] = true
    block_type["break_time"] = 1000
    block_type["ghost"] = true
    block_type["transparent"] = true
    block_type["effective_tool"] = tools.axe
    block_type["required_tool_power"] = 10
    blocks.wood = terralistic_register_block_type(block_type)

    -- BRANCH
    block_type = terralistic_new_block_type()
    block_type["name"] = "branch"
    block_type["break_time"] = 1000
    block_type["ghost"] = true
    block_type["transparent"] = true
    blocks.branch = terralistic_register_block_type(block_type)

    -- LEAVES
    block_type = terralistic_new_block_type()
    block_type["name"] = "leaves"
    block_type["ghost"] = true
    block_type["transparent"] = true
    blocks.leaves = terralistic_register_block_type(block_type)

    -- CANOPY
    block_type = terralistic_new_block_type()
    block_type["name"] = "canopy"
    block_type["ghost"] = true
    block_type["transparent"] = true
    block_type["width"] = 5
    block_type["height"] = 5
    blocks.canopy = terralistic_register_block_type(block_type)

    terralistic_connect_blocks(blocks.wood, blocks.canopy);

    -- GRASS
    block_type = terralistic_new_block_type()
    block_type["name"] = "grass"
    block_type["ghost"] = true
    block_type["transparent"] = true
    block_type["break_time"] = 0
    blocks.grass = terralistic_register_block_type(block_type)

    -- STONE
    block_type = terralistic_new_block_type()
    block_type["name"] = "stone"
    block_type["ghost"] = true
    block_type["transparent"] = true
    block_type["break_time"] = 0
    blocks.stone = terralistic_register_block_type(block_type)




    -- register walls
    wall_type = terralistic_new_wall_type()
    wall_type["name"] = "dirt"
    walls.dirt = terralistic_register_wall_type(wall_type)




    -- register items
    item_type = terralistic_new_item_type()
    item_type["name"] = "dirt"
    item_type["display_name"] = "Dirt Block"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.dirt
    items.dirt = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.dirt, items.dirt, 1)

    item_type = terralistic_new_item_type()
    item_type["name"] = "hatchet"
    item_type["display_name"] = "Hatchet"
    item_type["max_stack"] = 1
    item_type["tool"] = tools.axe
    item_type["tool_power"] = 10
    items.hatchet = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.grass_block, items.hatchet, 1)

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
    biome["base_block"] = blocks.dirt
    biome["base_wall"] = walls.dirt
    biome["generator_function"] = "generate_plains"
    biome:add_ore(blocks.stone_block, -2.0, 3.0);
    biome:add_ore(blocks.copper_ore, -0.9, -0.4);
    biomes.plains = terralistic_register_biome(biome)

    -- HILLS
    biome = terralistic_new_biome()
    biome["min_terrain_height"] = 10
    biome["max_terrain_height"] = 60
    biome["min_width"] = 100
    biome["max_width"] = 300
    biome["base_block"] = blocks.dirt
    biome["base_wall"] = walls.dirt
    biome["generator_function"] = "generate_plains"
    biome:add_ore(blocks.stone_block, -2.0, 3.0);
    biome:add_ore(blocks.copper_ore, -0.9, -0.4);
    biomes.hills = terralistic_register_biome(biome)
    terralistic_connect_biomes(biomes.plains, biomes.hills, 100)

    -- MOUNTAINS
    biome = terralistic_new_biome()
    biome["min_terrain_height"] = 40
    biome["max_terrain_height"] = 120
    biome["min_width"] = 100
    biome["max_width"] = 300
    biome["base_block"] = blocks.dirt
    biome["base_wall"] = walls.dirt
    biome:add_ore(blocks.stone_block, 1.0, 1.0);
    biome:add_ore(blocks.copper_ore, -0.42, -0.38);
    biomes.mountains = terralistic_register_biome(biome)
    terralistic_connect_biomes(biomes.hills, biomes.mountains, 100)
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("base_game mod stopped.")
end

-- This function is called every frame.
function update()

end

function on_block_break(x, y, block_id)
    if block_id == wood_block then
        -- if the wood block is broken, the wood block above it is also broken
        if terralistic_get_block(x, y - 1) == blocks.wood then
            terralistic_break_block(x, y - 1)
        end
        -- also break left and right wood blocks
        if terralistic_get_block(x - 1, y) == blocks.wood then
            terralistic_break_block(x - 1, y)
        end
        if terralistic_get_block(x + 1, y) == blocks.wood then
            terralistic_break_block(x + 1, y)
        end

        -- also break branch blocks on the left and right
        if terralistic_get_block(x - 1, y) == blocks.branch then
            terralistic_break_block(x - 1, y)
        end
        if terralistic_get_block(x + 1, y) == blocks.branch then
            terralistic_break_block(x + 1, y)
        end

        -- also break canopy_block 1 block above
        if terralistic_get_block(x, y - 1) == blocks.canopy then
            terralistic_break_block(x, y - 1)
        end
    end

    -- if branch_block is broken, break the leaves_block on the left and right
    if block_id == branch_block then
        if terralistic_get_block(x - 1, y) == blocks.leaves then
            terralistic_break_block(x - 1, y)
        end

        if terralistic_get_block(x + 1, y) == blocks.leaves then
            terralistic_break_block(x + 1, y)
        end
    end
end

function generate_plains(terrain, heights, width, height)
    for x = 1, width do
        if terrain[x][height - heights[x] + 1] == blocks.dirt then
            terrain[x][height - heights[x] + 1] = blocks.grass_block
        end
    end

    -- every 5 - 20 blocks there is a tree with a height of 7 - 20 blocks
    -- the tree is made of wood blocks, if there is the same height
    -- on the left or right, there is a 50% chance for each to spawn additional
    -- wood block on the left or right

    x = 3
    while x < width - 2 do
        tree_height = math.random(7, 15)
        tree_y = height - heights[x]
        if terrain[x][tree_y + 1] == blocks.grass_block then
            for y = tree_y - tree_height, tree_y do
                if y > 0 and y < height then
                    terrain[x][y] = blocks.wood
                end
            end

            if heights[x] == heights[x + 1] and terrain[x + 1][tree_y + 1] == blocks.grass_block and math.random(0, 1) == 0 then
                terrain[x + 1][tree_y] = blocks.wood
            end

            if heights[x] == heights[x - 1] and terrain[x - 1][tree_y + 1] == blocks.grass_block and math.random(0, 1) == 0 then
                terrain[x - 1][tree_y] = blocks.wood
            end

            leave_y = tree_y - math.random(3, 7)
            while leave_y > tree_y - tree_height do
                if terrain[x - 1][leave_y] == blocks.air and terrain[x - 2][leave_y] == blocks.air then
                    terrain[x - 1][leave_y] = blocks.branch
                    terrain[x - 2][leave_y] = blocks.leaves
                end
                leave_y = leave_y - math.random(3, 10);
            end

            leave_y = tree_y - math.random(3, 7)
            while leave_y > tree_y - tree_height do
                if terrain[x + 1][leave_y] == blocks.air and terrain[x + 2][leave_y] == blocks.air then
                    terrain[x + 1][leave_y] = blocks.branch
                    terrain[x + 2][leave_y] = blocks.leaves
                end
                leave_y = leave_y - math.random(3, 10);
            end

            terrain[x - 2][tree_y - tree_height - 5] = blocks.canopy
        end

        x = x + math.random(6, 15)
    end

    -- every 3 - 10 blocks there is a stone on the ground
    x = math.random(1, 8)
    while x < width - 1 do
        if terrain[x][height - heights[x] + 1] == blocks.grass_block and terrain[x][height - heights[x]] == blocks.air then
            terrain[x][height - heights[x]] = blocks.stone
        end

        x = x + math.random(3, 10)
    end

    -- every 3 - 10 blocks there is grass on the ground
    x = math.random(1, 8)
    while x < width - 1 do
        if terrain[x][height - heights[x] + 1] == blocks.grass_block and terrain[x][height - heights[x]] == blocks.air then
            terrain[x][height - heights[x]] = blocks.grass
        end

        x = x + math.random(3, 10)
    end

    return terrain
end
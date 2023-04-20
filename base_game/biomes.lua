biomes = {}

function register_biomes()
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
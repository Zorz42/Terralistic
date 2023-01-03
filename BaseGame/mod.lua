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
dirt = -1

-- global variables for biome IDs
plains = -1

-- This function is called when the mod is loaded.
function init()
    terralistic_print("BaseGame mod loaded.")
    block_type = terralistic_new_block_type()
    block_type["name"] = "dirt"
    dirt = terralistic_register_block_type(block_type)
end

-- This function is called when the mod is loaded on a server.
function init_server()
    terralistic_print("BaseGame mod loaded on server.")
    plains = terralistic_add_biome(1.0)
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("BaseGame mod stopped.")
end

-- This function is called when the mod is updated.
function update()

end

-- This function is called, when a world is being generated.
function generate_world(width, height, terrain)
    result = {}
    for x = 1, width do
        result[x] = {}
    end

    -- Iterate over all blocks in the terrain.
    for x = 1, width do
        for y = 1, height do
            -- if terrain is 0, set air, else set 10% dirt and 90% air
            if terrain[x][y] == 0 then
                result[x][y] = air
            else
                if math.random(0, 9) == 0 then
                    result[x][y] = dirt
                else
                    result[x][y] = air
                end
            end
        end
    end

    return result
end
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
VERSION = "1.0"

-- This function is called when the mod is loaded.
function init()
    register_tools()
    register_blocks()
    register_walls()
    register_items()
    register_recipes()

    terralistic_print("base_game mod loaded.")
end

-- This function is called when the mod is loaded on a server.
function init_server()
    register_biomes()
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("base_game mod stopped.")
end

-- This function is called every frame.
function update()

end

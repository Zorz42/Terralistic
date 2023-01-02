--[[

This is a Terralistic mod file.
It is used to define the mod's name,
description, version and the behavior
of the mod.

]]--


-- This function returns the mod's name.
function modName()
    return "BaseGame"
end

-- This function returns the mod's description.
function modDescription()
    return "The base game. It contains the basic"..
    "game mechanics and is required for all other"..
    "mods. It is always loaded."
end

-- This function returns the mod's version.
function modVersion()
    return "0.1"
end

-- This function is called when the mod is loaded.
function init()
    terralistic_print("BaseGame mod loaded.")
    block_type = terralistic_new_block_type()
    block_type["name"] = "dirt"
    terralistic_print("block name: \"" .. block_type["name"] .. "\"")
    terralistic_register_block_type(block_type)
end

-- This function is called when the mod is unloaded.
function stop()
    terralistic_print("BaseGame mod stopped.")
end

-- This function is called when the mod is updated.
function update()

end
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
    tprint("BaseGame mod loaded.")
end



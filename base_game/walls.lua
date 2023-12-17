walls = {}

function register_walls()
    terralistic_print("registering walls...")

    -- DIRT
    walls.dirt = terralistic_register_wall_type("dirt", 2000)

    -- WOOD_PLANKS
    walls.wood_planks = terralistic_register_wall_type("wood_planks", 2000)
end
walls = {}

function register_walls()
    terralistic_print("registering walls...")
    
    -- DIRT
    wall_type = terralistic_new_wall_type()
    wall_type["name"] = "dirt"
    walls.dirt = terralistic_register_wall_type(wall_type)
end
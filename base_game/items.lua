items = {}

function register_items()
    terralistic_print("registering items...")

    -- DIRT
    item_type = terralistic_new_item_type()
    item_type["name"] = "dirt"
    item_type["display_name"] = "Dirt Block"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.dirt
    items.dirt = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.dirt, items.dirt, 1)

    -- STONE
    item_type = terralistic_new_item_type()
    item_type["name"] = "stone"
    item_type["display_name"] = "Stone"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.stone
    items.stone = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.stone, items.stone, 1)

    -- FIBER
    item_type = terralistic_new_item_type()
    item_type["name"] = "fiber"
    item_type["display_name"] = "Fiber"
    item_type["max_stack"] = 99
    items.fiber = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.grass, items.fiber, 1)

    -- HATCHET
    item_type = terralistic_new_item_type()
    item_type["name"] = "hatchet"
    item_type["display_name"] = "Hatchet"
    item_type["max_stack"] = 1
    item_type["tool"] = tools.axe
    item_type["tool_power"] = 10
    items.hatchet = terralistic_register_item_type(item_type)

    -- HAMMER
    item_type = terralistic_new_item_type()
    item_type["name"] = "hammer"
    item_type["display_name"] = "Hammer"
    item_type["max_stack"] = 1
    item_type["tool"] = tools.hammer
    item_type["tool_power"] = 10
    items.hammer = terralistic_register_item_type(item_type)
    
    -- PICKAXE
    item_type = terralistic_new_item_type()
    item_type["name"] = "pickaxe"
    item_type["display_name"] = "Pickaxe"
    item_type["max_stack"] = 1
    item_type["tool"] = tools.pickaxe
    item_type["tool_power"] = 10
    items.pickaxe = terralistic_register_item_type(item_type)
    
    -- SHOVEL
    item_type = terralistic_new_item_type()
    item_type["name"] = "shovel"
    item_type["display_name"] = "Shovel"
    item_type["max_stack"] = 1
    item_type["tool"] = tools.shovel
    item_type["tool_power"] = 10
    items.shovel = terralistic_register_item_type(item_type)

    -- BRANCH
    item_type = terralistic_new_item_type()
    item_type["name"] = "branch"
    item_type["display_name"] = "Branch"
    item_type["max_stack"] = 99
    items.branch = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.branch, items.branch, 1)

    -- WOOD PLANKS
    item_type = terralistic_new_item_type()
    item_type["name"] = "wood_planks"
    item_type["display_name"] = "Wood planks"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.wood_planks
    items.wood_planks = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.wood, items.wood_planks, 1)
    terralistic_set_block_drop(blocks.wood_planks, items.wood_planks, 1)

    -- WOOD PLANK WALL
    item_type = terralistic_new_item_type()
    item_type["name"] = "wood_plank_wall"
    item_type["display_name"] = "Wood plank wall"
    item_type["max_stack"] = 99
    items.wood_plank_wall = terralistic_register_item_type(item_type)

    -- TORCH
    item_type = terralistic_new_item_type()
    item_type["name"] = "torch"
    item_type["display_name"] = "Torch"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.torch
    items.torch = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.torch, items.torch, 1)
    
    -- STONE BLOCK
    item_type = terralistic_new_item_type()
    item_type["name"] = "stone_block"
    item_type["display_name"] = "Stone Block"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.stone_block
    items.stone_block = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.stone_block, items.stone_block, 1)
    
    -- COPPER ORE
    item_type = terralistic_new_item_type()
    item_type["name"] = "copper_ore"
    item_type["display_name"] = "Copper Ore"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.copper_ore
    items.copper_ore = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.copper_ore, items.copper_ore, 1)
    
    -- FURNACE
    item_type = terralistic_new_item_type()
    item_type["name"] = "furnace"
    item_type["display_name"] = "Furnace"
    item_type["max_stack"] = 99
    item_type["places_block"] = blocks.furnace
    items.furnace = terralistic_register_item_type(item_type)
    terralistic_set_block_drop(blocks.furnace, items.furnace, 1)
end
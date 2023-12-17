items = {}

function register_items()
    terralistic_print("registering items...")

    -- DIRT
    items.dirt = terralistic_register_item_type(
            -- name
            "dirt",
            -- display_name
            "Dirt Block",
            -- max_stack
            99,
            -- places_block
            blocks.dirt,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.dirt, items.dirt, 1)

    -- STONE
    items.stone = terralistic_register_item_type(
            -- name
            "stone",
            -- display_name
            "Stone",
            -- max_stack
            99,
            -- places_block
            blocks.stone,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.stone, items.stone, 1)

    -- FIBER
    items.fiber = terralistic_register_item_type(
            -- name
            "fiber",
            -- display_name
            "Fiber",
            -- max_stack
            99,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.grass, items.fiber, 1)

    -- HATCHET
    items.hatchet = terralistic_register_item_type(
            -- name
            "hatchet",
            -- display_name
            "Hatchet",
            -- max_stack
            1,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            tools.axe,
            -- tool_power
            10
    )

    -- HAMMER
    items.hammer = terralistic_register_item_type(
            -- name
            "hammer",
            -- display_name
            "Hammer",
            -- max_stack
            1,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            tools.hammer,
            -- tool_power
            10
    )
    
    -- PICKAXE
    items.pickaxe = terralistic_register_item_type(
            -- name
            "pickaxe",
            -- display_name
            "Pickaxe",
            -- max_stack
            1,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            tools.pickaxe,
            -- tool_power
            10
    )
    
    -- SHOVEL
    items.shovel = terralistic_register_item_type(
            -- name
            "shovel",
            -- display_name
            "Shovel",
            -- max_stack
            1,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            tools.shovel,
            -- tool_power
            10
    )

    -- BRANCH
    items.branch = terralistic_register_item_type(
            -- name
            "branch",
            -- display_name
            "Branch",
            -- max_stack
            99,
            -- places_block
            nil,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.branch, items.branch, 1)

    -- WOOD PLANKS
    items.wood_planks = terralistic_register_item_type(
            -- name
            "wood_planks",
            -- display_name
            "Wood planks",
            -- max_stack
            99,
            -- places_block
            blocks.wood_planks,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.wood, items.wood_planks, 1)
    terralistic_set_block_drop(blocks.wood_planks, items.wood_planks, 1)

    -- WOOD PLANK WALL
    items.wood_plank_wall = terralistic_register_item_type(
            -- name
            "wood_plank_wall",
            -- display_name
            "Wood plank wall",
            -- max_stack
            99,
            -- places_block
            nil,
            -- places_wall
            walls.wood_planks,
            -- tool
            nil,
            -- tool_power
            0
    )

    -- TORCH
    items.torch = terralistic_register_item_type(
            -- name
            "torch",
            -- display_name
            "Torch",
            -- max_stack
            99,
            -- places_block
            blocks.torch,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.torch, items.torch, 1)
    
    -- STONE BLOCK
    items.stone_block = terralistic_register_item_type(
            -- name
            "stone_block",
            -- display_name
            "Stone Block",
            -- max_stack
            99,
            -- places_block
            blocks.stone_block,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.stone_block, items.stone_block, 1)
    
    -- COPPER ORE
    items.copper_ore = terralistic_register_item_type(
            -- name
            "copper_ore",
            -- display_name
            "Copper Ore",
            -- max_stack
            99,
            -- places_block
            blocks.copper_ore,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.copper_ore, items.copper_ore, 1)
    
    -- FURNACE
    items.furnace = terralistic_register_item_type(
            -- name
            "furnace",
            -- display_name
            "Furnace",
            -- max_stack
            99,
            -- places_block
            blocks.furnace,
            -- places_wall
            nil,
            -- tool
            nil,
            -- tool_power
            0
    )
    terralistic_set_block_drop(blocks.furnace, items.furnace, 1)
end
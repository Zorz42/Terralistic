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
    terralistic_set_block_drop(blocks.grass_block, items.hatchet, 1)
end
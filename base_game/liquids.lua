liquids = {}

function register_liquids()
    terralistic_print("registering liquids...")

    -- WATER
    liquids.water = terralistic_register_liquid_type(
            -- name
            "water",
            -- flow_time
            100,
            -- speed_multiplier
            0.4
    )
end

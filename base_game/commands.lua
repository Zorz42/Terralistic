function command_give(arguments, executor)
    if #arguments < 1 then
        return "Command 'give' requires at least one argument."
    end
    
    if #arguments > 3 then
        return "Command 'give' takes at most three arguments."
    end
    
    player = executor
    item_name = arguments[1]
    
    item = terralistic_get_item_id_by_name(item_name)
    if item == nil then
        return "Item '" .. item_name .. "' not found."
    end
    
    amount = 1
    if #arguments >= 2 then
        amount = tonumber(arguments[2])
        if amount == nil then
            return "Argument 2 must be a number."
        end
    end
    
    if #arguments >= 3 then
        player = terralistic_get_player_by_name(arguments[3])
    end
    
    if player == nil then
        return "Player '" .. arguments[3] .. "' not found."
    end
    
    terralistic_give_item(player, item, amount)
    
    return "Gave item"
end

function describe_command_give() 
    return 
[[Gives the player an item.
Usage: give <item> [amount] [player] - amount defaults to 1, player defaults to the executor.]]
end

function command_stop(arguments, executor)
    if #arguments ~= 0 then
        return "Command 'stop' does not take any arguments."
    end
    
    terralistic_stop_server()
    return "Stopping the server..."
end

function describe_command_stop()
    return "Stops the server."
end
function command_give(arguments, executor)
    return "command not implemented ;)"
end

function describe_command_give() 
    return "Gives the player an item."
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
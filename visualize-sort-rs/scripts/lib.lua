---@diagnostic disable-next-line: lowercase-global
algorithms = {
    quicksort = require("scripts.quicksort"),
}

function execute(algorithm_name, values)
    -- Initial
    array.snapshot(values, {}, {});

    algorithms[algorithm_name].sort(values)

    -- Clear accesses and writes
    array.snapshot(values, {}, {});
    return values.history
end

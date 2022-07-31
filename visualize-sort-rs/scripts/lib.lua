---@diagnostic disable-next-line: lowercase-global
algorithms = {
    quicksort = require("scripts.quicksort"),
}

function execute(algorithm_name, values)
    algorithms[algorithm_name].sort(values)
    print(values)
end

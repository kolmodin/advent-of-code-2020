input = open("../inputs/day01.txt") do f
    read(f, String)
end

function parseInput(inp::String)::Array{Int64,1}
    [parse(Int64, ln) for ln in split(inp, "\n")]
end

function main()
    set = parseInput(input)
    for a in set
        for b in set
            c = 2020 - a - b
            if c in set
                println("$a + $b + $c = 2020")
                println("$a * $b * $c = $(a * b * c)")
                return
            end
        end
    end
end

main()
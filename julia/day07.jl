lines = open("../inputs/day07.txt") do f
    readlines(f)
end

struct BagCount
    name::String
    count::Int64
end

function parse_line(line::String)::Tuple{String,Vector{BagCount}}
    ln = rstrip(line, '.')
    parts = split(ln, " bags contain ")
    @assert (length(parts) == 2)
    name = parts[1]
    contents = parts[2]
    if contents == "no other bags"
        return (name, [])
    end

    return (name, [
        begin
        parts = split(bag, " ", limit=2)
        name = split(parts[2], " bag")[1]
        count = parse(Int64, parts[1])
        BagCount(name, count)
    end
        for bag in split(contents, ", ")
    ])
end

function canHoldMyBag(someBag, myBag, memory, bagDict)::Bool
    prev = get(memory, someBag, nothing)
    if prev !== nothing
        return prev
    end
    for bag in bagDict[someBag]
        if bag.name == myBag || canHoldMyBag(bag.name, myBag, memory, bagDict)
            memory[someBag] = true
            return true
        end
    end
    memory[someBag] = false
    return false
end

function bagCounter(someBag, memory, bagDict)::Int64
    prev = get(memory, someBag, nothing)
    if prev !== nothing
        return prev
    end
    return 1 + sum([ bag.count * bagCounter(bag.name, memory, bagDict)
                     for bag in bagDict[someBag] ])
end

function part1(bagDict, myBag)
    memory = Dict{String,Bool}()
    return length([ ()
                    for name in keys(bagDict)
                    if canHoldMyBag(name, myBag, memory, bagDict)])
end

function part2(bagDict, myBag)
    memory = Dict{String,Int64}()
    return bagCounter(myBag, memory, bagDict) - 1
    
end

myBag = "shiny gold"
bags = parse_line.(lines)
bagDict = Dict{String,Vector{BagCount}}(
    name => contents
    for (name, contents) in bags)

println("Part 1: ", part1(bagDict, myBag))
println("Part 2: ", part2(bagDict, myBag))
# time ./julia day08.jl -> 255ms when using a single type to represent
# all operations:
# struct Instr
#  op :: Symbol
#  num :: Int64
# end
#
# Increases to 347ms when using separate types for
# each op (OpNop, OpJmp, OpAcc) and Union{..} for an instruction.

lines = open("../inputs/day08.txt") do f
    readlines(f)
end

# Instructions
struct OpNop
    num::Int64
end
struct OpJmp
    num::Int64
end
struct OpAcc
    num::Int64
end

isNop(i::OpNop) = true
isNop(_) = false

isJmp(i::OpJmp) = true
isJmp(_) = false

isAcc(i::OpAcc) = true
isAcc(_) = false

Instr = Union{OpNop,OpJmp,OpAcc}

# Possible terminations
struct Terminated
    acc::Int64
end

isTerminated(t::Terminated) = true
isTerminated(_) = false

struct Seen
    acc::Int64
end

# Parse

function parseLine(ln::String)::Instr
    parts = split(ln, " ")
    num = parse(Int64, parts[2])
    if parts[1] == "nop"
        return OpNop(num)
    elseif parts[1] == "jmp"
        return OpJmp(num)
    else
        @assert parts[1] == "acc"
        return OpAcc(num)
    end
end

function run(program::Vector{Instr})::Union{Terminated,Seen}
    acc = 0
    pc = 1
    seen = Set{Int64}()
    term_pc = length(program) + 1
    while pc ∉ seen && pc != term_pc
        push!(seen, pc)
        op = program[pc]
        if isNop(op)
            pc += 1
            continue
        elseif isJmp(op)
            pc += op.num
            continue
        else
            @assert isAcc(op)
            acc += op.num
            pc += 1
        end
    end
    if pc == term_pc
        return Terminated(acc)
    else
        return Seen(acc)
    end
end

part1(program) = run(program).acc

function part2(program)::Int64
    for i in 1:length(program)
        op = program[i]
        if isJmp(op)
            newOp = OpNop(op.num)
        elseif isNop(op)
            newOp = OpJmp(op.num)
        else
            continue
        end
        program[i] = newOp
        code = run(program)
        if isTerminated(code)
            return code.acc
        end
        program[i] = op
    end
end


function main()
    program::Vector{Instr} = parseLine.(lines)
    println("Part 1: ", part1(program))
    println("Part 2: ", part2(program))
end

main()
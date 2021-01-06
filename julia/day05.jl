input = open("../inputs/day05.txt") do f
    read(f, String)
end

function range(s, low, high)
    lo = low
    hi = high
    for c in s
        if c == 'F' || c == 'L'
            hi = lo + (hi - lo) ÷ 2
        elseif c == 'B' || c == 'R'
            lo = lo + (hi - lo + 1) ÷ 2
        end
    end
    @assert (lo == hi)
    lo
end

function seat_id(s)
    row = range(s[1:7], 0, 127)
    col = range(s[8:10], 0, 7)
    row * 8 + col
end

all_seats = [ seat_id(ln) for ln in split(input, "\n")]
sort!(all_seats)

println("Part 1: ", max(all_seats...))

my_seat_id = first(
    [ all_seats[i] - 1
    for i in 2:length(all_seats)
    if all_seats[i - 1] + 1 != all_seats[i]])

println("Part 2: ", my_seat_id)
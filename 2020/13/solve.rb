# frozen_string_literal: true

def parse_bus_schedule(string)
  head, tail = string.split("\n")

  timestamp = head.to_i
  buses = tail.split(',').keep_if { |s| s != 'x' }.map(&:to_i)

  [timestamp, buses]
end

def earliest_bus(buses, timestamp)
  depature_times = buses.map do |bus|
    remainder, = timestamp.divmod(bus)
    next_stop = (remainder + 1) * bus

    [bus, next_stop]
  end

  depature_times.min_by(&:last)
end

def part1(input)
  timestamp, buses = parse_bus_schedule(input)
  bus, departure_time = earliest_bus(buses, timestamp)
  bus * (departure_time - timestamp)
end

def parse_bus_schedule_with_index(string)
  _, tail = string.split("\n")

  tail.split(',').filter_map.with_index do |bus, i|
    next if bus == 'x'

    n = bus.to_i
    [n, (n - i) % n]
  end
end

def part2(input)
  buses = parse_bus_schedule_with_index(input)

  timestamp = buses[0][1]
  offset = buses[0][0]
  i = 0

  while i < buses.size - 1
    next_bus_id, next_bus_wait = buses[i + 1]

    until timestamp % next_bus_id == next_bus_wait
      timestamp += offset
    end

    offset *= next_bus_id

    i += 1
  end

  timestamp
end

demo_input = <<~TXT
  939
  7,13,x,x,59,x,31,19
TXT

pp part1(demo_input) == 295

puzzle_input = ARGF.read

puts "Puzzle answer a: #{part1(puzzle_input)}"
puts "Puzzle answer b: #{part2(puzzle_input)}"

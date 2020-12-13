# frozen_string_literal: true

def string_to_grid(input)
  input.split("\n").map(&:chars)
end

def grid_to_string(grid)
  grid.map(&:join).join("\n")
end

def bounds(grid)
  bx = grid[0].size - 1
  by = grid.size - 1

  [bx, by]
end

def count_seats(grid, state = '#')
  grid.flatten.count { |char| char == state }
end

NEIGHBOR_DELTAS =
  [-1, 0, 1]
  .repeated_permutation(2)
  .to_a
  .keep_if { |(x, y)| !(x.zero? && y.zero?) }

def occupied_neighbors(grid, x, y)
  bx, by = bounds(grid)

  NEIGHBOR_DELTAS
    .map { |(dx, dy)| [x + dx, y + dy] }
    .keep_if { |(x, y)| x >= 0 && x <= bx && y >= 0 && y <= by }
    .map { |(x, y)| grid[y][x] }
    .keep_if { |char| char == '#' }
    .size
end

DIRECTIONS = [
  [-1, -1], # UP LEFT
  [-1, 0],  # UP
  [-1, 1],  # UP RIGHT
  [0, -1],  # LEFT
  [0, 1],   # RIGHT
  [1, -1],  # DOWN LEFT
  [1, 0],   # DOWN
  [1, 1]    # DOWN RIGHT
]

def occupied_line_of_sight(grid, col, row)
  bx, by = bounds(grid)

  result = 0

  DIRECTIONS.each do |(dx, dy)|
    x = col
    y = row

    loop do
      x += dx
      y += dy

      break unless x >= 0 && x <= bx && y >= 0 && y <= by

      char = grid[y][x]

      next if char == '.'

      result += 1 if char == '#'

      break
    end
  end

  result
end

def evolve_a(grid, bx, by)
  g = grid.map(&:dup)

  (0..by).each do |y|
    (0..bx).each do |x|
      seat = grid[y][x]
      g[y][x] = '#' if seat == 'L' && occupied_neighbors(grid, x, y).zero?
      g[y][x] = 'L' if seat == '#' && occupied_neighbors(grid, x, y) >= 4
    end
  end

  g
end

def evolve_b(grid, bx, by)
  g = grid.map(&:dup)

  (0..by).each do |y|
    (0..bx).each do |x|
      seat = grid[y][x]
      g[y][x] = '#' if seat == 'L' && occupied_line_of_sight(grid, x, y).zero?
      g[y][x] = 'L' if seat == '#' && occupied_line_of_sight(grid, x, y) >= 5
    end
  end

  g
end

def simulate(grid:, part: :a, verbose: false)
  state = grid
  generation = 0

  bx, by = bounds(grid)

  loop do
    if verbose
      puts "Gen #{generation}\n\n"
      puts grid_to_string(state)
      puts ''
    end

    next_state = if part == :a
                   evolve_a(state, bx, by)
                 else
                   evolve_b(state, bx, by)
                 end

    break if next_state.flatten == state.flatten

    state = next_state
    generation += 1
  end

  state
end

def answers(input, verbose: false)
  g0 = string_to_grid(input)

  g1 = simulate(grid: g0, part: :a)
  puts grid_to_string(g1)
  puts "A: #{count_seats(g1, '#')}"

  g2 = simulate(grid: g0, part: :b)
  puts grid_to_string(g2)
  puts "B: #{count_seats(g2, '#')}"
end

a = <<~TXT
  .......#.
  ...#.....
  .#.......
  .........
  ..#L....#
  ....#....
  .........
  #........
  ...#.....
TXT

b = <<~TXT
  .............
  .L.L.#.#.#.#.
  .............
TXT

c = <<~TXT
  .##.##.
  #.#.#.#
  ##...##
  ...L...
  ##...##
  #.#.#.#
  .##.##.
TXT

ga = string_to_grid(a)
gb = string_to_grid(b)
gc = string_to_grid(c)

pp occupied_line_of_sight(ga, 3, 4) == 8
pp occupied_line_of_sight(gb, 1, 1) == 0
pp occupied_line_of_sight(gc, 3, 3) == 0

demo_input = <<~TXT
  L.LL.LL.LL
  LLLLLLL.LL
  L.L.L..L..
  LLLL.LL.LL
  L.LL.LL.LL
  L.LLLLL.LL
  ..L.L.....
  LLLLLLLLLL
  L.LLLLLL.L
  L.LLLLL.LL
TXT

puts 'Demo'
answers(demo_input)

puzzle_input = ARGF.read

puts 'Puzzle'
answers(puzzle_input)

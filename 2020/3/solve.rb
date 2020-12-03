# frozen_string_literal: true

def trees(grid, dx = 1, dy = 3)
  x = 0
  y = 0
  count = 0

  while y + 1 < grid.length
    x += dx
    y += dy

    count += 1 if grid[y][x % grid[y].length] == '#'
  end

  count
end

grid = ARGF.readlines.map(&:strip).map(&:chars)
slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]

puts trees(grid, 3, 1)
puts slopes.map { |(x, y)| trees(grid, x, y) }.reduce(:*)

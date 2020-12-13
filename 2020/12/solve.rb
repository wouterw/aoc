# frozen_string_literal: true

DIRECTIONS = {
  'N' => [0, 1],
  'E' => [-1, 0],
  'S' => [0, -1],
  'W' => [1, 0]
}.freeze

ANGLES = {
  0 => 'E',
  90 => 'N',
  180 => 'W',
  270 => 'S'
}.freeze

def parse_navigation_instructions(string)
  string.split("\n").map(&:strip).map do |s|
    match = s.match(/^(\w)(\d+)$/)
    [match[1], match[2].to_i]
  end
end

def navigate_a(instructions)
  x = 0
  y = 0
  angle = 0

  instructions.each do |(action, value)|
    case action
    when 'L'
      degrees = value
      angle = (angle + degrees) % 360
    when 'R'
      degrees = 360 - value
      angle = (angle + degrees) % 360
    when 'N', 'E', 'S', 'W'
      dx, dy = DIRECTIONS[action]
      x += value * dx
      y += value * dy
    when 'F'
      dx, dy = DIRECTIONS[ANGLES[angle]]
      x += value * dx
      y += value * dy
    end
  end

  [x, y]
end

P = Struct.new(:x, :y) do
  def move(dx, dy)
    self.x += dx
    self.y += dy
  end

  def rotate_r(degree)
    nx, ny = case degree
             when 90 then [-y, x]
             when 180 then [-x, -y]
             when 270 then [y, -x]
             end

    self.x = nx
    self.y = ny
  end

  def rotate_l(degree)
    nx, ny = case degree
             when 90 then [y, -x]
             when 180 then [-x, -y]
             when 270 then [-y, x]
             end

    self.x = nx
    self.y = ny
  end

  def manhattan_distance
    x.abs + y.abs
  end

  def to_s
    "(#{x}, #{y})"
  end
end

def navigate_b(instructions)
  waypoint = P.new(1, 10)
  location = P.new(0, 0)

  puts "WP #{waypoint} - SP #{location}"

  instructions.each do |(action, n)|
    case action
    when 'N'
      waypoint.move(n, 0)
    when 'E'
      waypoint.move(0, n)
    when 'S'
      waypoint.move(-n, 0)
    when 'W'
      waypoint.move(0, -n)
    when 'R'
      waypoint.rotate_r(n)
    when 'L'
      waypoint.rotate_l(n)
    when 'F'
      location.move(waypoint.x * n, waypoint.y * n)
    end

    puts "#{action} #{n} -> WP #{waypoint} - SP #{location}"
  end

  location
end

def part1(input)
  instructions = parse_navigation_instructions(input)
  x, y = navigate_a(instructions)
  x.abs + y.abs
end

def part2(input)
  instructions = parse_navigation_instructions(input)
  location = navigate_b(instructions)
  location.manhattan_distance
end

demo_input = <<~TXT
  F10
  N3
  F7
  R90
  F11
TXT

pp part1(demo_input) == 25
pp part2(demo_input) == 286

puzzle_input = ARGF.read

puts "Puzzle answer a: #{part1(puzzle_input)}"
puts "Puzzle answer b: #{part2(puzzle_input)}"

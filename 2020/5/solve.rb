# frozen_string_literal: true

require 'set'

def seat_id(boarding_pass)
  row = find_row(boarding_pass[..6])
  column = find_column(boarding_pass[-3..])

  row * 8 + column
end

def find_row(steps)
  find_index(0..127, steps)
end

def find_column(steps)
  find_index(0..7, steps)
end

def find_index(range, steps)
  result = steps.reduce(range) do |memo, step|
    lower, upper = memo.each_slice(memo.count / 2).to_a

    case step
    when 'R', 'B' then upper
    when 'L', 'F' then lower
    end
  end

  result.first
end

boarding_passes = ARGF.readlines.map(&:strip).map(&:chars)

seat_ids = boarding_passes.map { |boarding_pass| seat_id(boarding_pass) }.sort

highest_seat_id = seat_ids.max

puts "Puzzle answer a: #{highest_seat_id}"

a = Set.new(seat_ids.first..seat_ids.last)
b = Set.new(seat_ids)

my_seat_id = (a - b).first

puts "Puzzle answer b: #{my_seat_id}"

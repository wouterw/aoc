# frozen_string_literal: true

require 'set'

def seat_id(boarding_pass)
  boarding_pass.tr('BFLR', '1001').to_i(2)
end

boarding_passes = ARGF.readlines.map(&:strip)

seat_ids = boarding_passes.map { |boarding_pass| seat_id(boarding_pass) }.sort

highest_seat_id = seat_ids.max

puts "Puzzle answer a: #{highest_seat_id}"

a = Set.new(seat_ids.first..seat_ids.last)
b = Set.new(seat_ids)

my_seat_id = (a - b).first

puts "Puzzle answer b: #{my_seat_id}"

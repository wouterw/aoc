# frozen_string_literal: true

require 'set'

groups = ARGF.read.split("\n\n")

number_of_questions_anyone = groups
  .map { |line| line.gsub("\n", '') }
  .map { |answers| Set.new(answers.chars) }
  .map(&:count)
  .reduce(:+)

number_of_questions_everyone = groups
  .map do |line|
    line
      .split("\n")
      .map { |answers| Set.new(answers.chars) }
      .reduce(:&)
      .count
  end
  .reduce(:+)

puts "Puzzle answer a: #{number_of_questions_anyone}"
puts "Puzzle answer b: #{number_of_questions_everyone}"

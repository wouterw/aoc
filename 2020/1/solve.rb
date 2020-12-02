# frozen_string_literal: true

require 'set'

def find_pair(expense_report, sum = 2020)
  term = expense_report.find do |expense|
    expense_report.include?(sum - expense)
  end

  return [] if term.nil?

  [term, sum - term]
end

def find_triplet(expense_report, sum = 2020)
  triplet = []

  expense_report.each do |expense|
    term = sum - expense
    pair = find_pair(expense_report.subtract([expense]), term)
    triplet = [expense, pair.first, pair.last] if pair.count.positive?
  end

  triplet
end

def product(enumerable)
  enumerable.inject(1) { |e, product| product * e }
end

input = Set.new(ARGF.readlines.map(&:to_i))

pair = find_pair(input)
puts product(pair)

triplet = find_triplet(input)
puts product(triplet)

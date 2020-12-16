# frozen_string_literal: true

require 'set'

def parse_ticket(ticket)
  ticket.split(',').map(&:to_i)
end

def parse_rules(rules = [])
  rules.each_with_object({}) do |rule, memo|
    match = rule.match(/^(.+): (\d+)-(\d+) or (\d+)-(\d+)$/).to_a
    memo[match[1]] = [match[2..3].map(&:to_i), match[4..5].map(&:to_i)]
  end
end

def parse_document(input)
  parts = input.split("\n\n")

  rules = parse_rules(parts[0].split("\n"))
  our_ticket = parse_ticket(parts[1].split("\n").pop)
  nearby_tickets = parts[2].split("\n").drop(1).map { |ticket| parse_ticket(ticket) }

  [rules, our_ticket, nearby_tickets]
end

def ticket_scanning_error_rate(input)
  rules, _our_ticket, nearby_tickets = parse_document(input)
  ranges = rules.values.flatten(1)

  invalid_values = nearby_tickets.flatten.filter do |value|
    ranges.none? { |(min, max)| value.between?(min, max) }
  end

  invalid_values.sum
end

def valid_tickets(tickets, rules)
  ranges = rules.values.flatten(1)

  tickets.reject do |ticket|
    ticket.any? do |value|
      ranges.none? { |(min, max)| value.between?(min, max) }
    end
  end
end

def decode_ticket(our_ticket, valid_tickets, rules)
  valid_positions_per_rule = rules.map do |(name, ranges)|
    positions = (0...rules.size).select do |i|
      valid_tickets.all? do |ticket|
        ranges.any? { |r| ticket[i].between?(r.min, r.max) }
      end
    end

    [name, positions]
  end

  field_order = valid_positions_per_rule
    .sort_by { |_, positions| positions.count }
    .each_with_object({}) do |(name, positions), memo|
      taken_positions = Set.new(memo.values)
      available_positions = positions.reject { |position| taken_positions.include?(position) }
      memo[name] = available_positions.first
    end

  field_order.each_with_object({}) do |(f, i), memo|
    memo[f] = our_ticket[i]
  end
end

def departure_field_product(input)
  rules, our_ticket, nearby_tickets = parse_document(input)

  valid_nearby_tickets = valid_tickets(nearby_tickets, rules)

  decoded_ticket = decode_ticket(our_ticket, valid_nearby_tickets, rules)

  decoded_ticket.reduce(1) do |product, (k, v)|
    if k.start_with?('departure')
      product * v
    else
      product
    end
  end
end

sample_input = <<~DOC
  class: 1-3 or 5-7
  row: 6-11 or 33-44
  seat: 13-40 or 45-50

  your ticket:
  7,1,14

  nearby tickets:
  7,3,47
  40,4,50
  55,2,20
  38,6,12
DOC

another_sample_input = <<~DOC
  class: 1-3 or 5-7
  row: 6-11 or 33-44
  seat: 13-40 or 45-50

  your ticket:
  7,1,14

  nearby tickets:
  7,3,47
  40,4,50
  55,2,20
  38,6,12
DOC

pp ticket_scanning_error_rate(sample_input) == 71
# pp departure_field_product(another_sample_input)

puzzle_input = ARGF.read

pp ticket_scanning_error_rate(puzzle_input)
pp departure_field_product(puzzle_input)

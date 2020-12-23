# frozen_string_literal: true

def score(deck:)
  deck
    .reverse
    .map
    .with_index { |card, index| card * (index + 1) }
    .sum
end

def game(input)
  deck_a, deck_b = input.split("\n\n").map do |part|
    part.split("\n").drop(1).map(&:to_i)
  end

  winner = nil

  while !deck_a.empty? && !deck_b.empty?
    card_a = deck_a.shift
    card_b = deck_b.shift

    winner = if card_a > card_b
               deck_a
             elsif card_b > card_a
               deck_b
             end

    min, max = [card_a, card_b].minmax

    winner.append(max)
    winner.append(min)
  end

  score(deck: winner)
end

sample_input = <<~TXT
  Player 1:
  9
  2
  6
  3
  1

  Player 2:
  5
  8
  4
  7
  10
TXT

warn 'Assert failed!' if game(sample_input) != 306

puzzle_input = ARGF.read

game(puzzle_input)

# frozen_string_literal: true

require 'set'

def parse(input)
  input.split("\n").map do |line|
    match = line.match(/((?:\w+ )+)\(contains ((?:\w+(?:, )?)+)\)/)
    [match[1].split(' '), match[2].split(', ')]
  end
end

def part1(input)
  list_of_foods = parse(input)

  food_sets = []
  set_of_ingredients = Set.new
  foods_by_allergen = Hash.new { |h, k| h[k] = [] }

  list_of_foods.each do |(ingredients, allergens)|
    food = Set.new(ingredients)

    food_sets << food
    set_of_ingredients.merge(ingredients)

    allergens.each do |allergen|
      foods_by_allergen[allergen] << food
    end
  end

  ingredients_by_allergen = foods_by_allergen.each_with_object({}) do |(allergen, foods), memo|
    memo[allergen] = foods.reduce { |set, food| set & food }
  end

  allergen_ingredients = ingredients_by_allergen.values.reduce { |memo, set| memo | set }
  non_allergen_ingredients = set_of_ingredients - allergen_ingredients

  food_sets.reduce(0) do |sum, food|
    sum + non_allergen_ingredients.map { |allergen| food.include?(allergen) ? 1 : 0 }.sum
  end
end

def part2(input)
  list_of_foods = parse(input)

  food_sets = []
  set_of_ingredients = Set.new
  foods_by_allergen = Hash.new { |h, k| h[k] = [] }

  list_of_foods.each do |(ingredients, allergens)|
    food = Set.new(ingredients)

    food_sets << food
    set_of_ingredients.merge(ingredients)

    allergens.each do |allergen|
      foods_by_allergen[allergen] << food
    end
  end

  ingredients_by_allergen = foods_by_allergen.each_with_object({}) do |(allergen, foods), memo|
    memo[allergen] = foods.reduce { |set, food| set & food }
  end

  allergen_ing = []
  remaining_allergens = ingredients_by_allergen.map { |k, v| [k, v] }

  while remaining_allergens.any?
    remaining_allergens.sort_by! { |(_, allergens)| allergens.count }

    ingredient, allergens = remaining_allergens.shift
    allergen = allergens.to_a[0]
    allergen_ing << [ingredient, allergen]

    remaining_allergens.each do |(_, allergens)|
      allergens.delete(allergen)
    end
  end

  allergen_ing.sort_by { |allergen, _| allergen }.map(&:last).join(',')
end

sample_input = <<~TXT
  mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
  trh fvjkl sbzzf mxmxvkd (contains dairy)
  sqjhc fvjkl (contains soy)
  sqjhc mxmxvkd sbzzf (contains fish)
TXT

puts part1(sample_input) == 5
puts part2(sample_input) == 'mxmxvkd,sqjhc,fvjkl'

puzzle_input = ARGF.read

puts part1(puzzle_input)
puts part2(puzzle_input)

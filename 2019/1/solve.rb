# frozen_string_literal: true

# module_mass = ARGF.readlines

# def fuel_requirement(mass)
#   (mass.to_f / 3.0).floor - 2.0
# end

# total_fuel_requirement = module_mass.sum { |e| fuel_requirement(e) }

# puts total_fuel_requirement

# require 'minitest/autorun'

# class Test < Minitest::Test
#   def test_fuel_requirement
#     assert_equal 2, fuel_requirement(12)
#     assert_equal 2, fuel_requirement(14)
#     assert_equal 654, fuel_requirement(1969)
#     assert_equal 33_583, fuel_requirement(100_756)
#   end
# end

def fuel_requirement(mass)
  fuel = (mass.to_f / 3.0).floor - 2.0

  if fuel.negative? || fuel.nil?
    0
  else
    fuel + fuel_requirement(fuel)
  end
end

require 'minitest/autorun'

class Test < Minitest::Test
  def test_fuel_requirement
    assert_equal 2, fuel_requirement(12)
    assert_equal 2, fuel_requirement(14)
    assert_equal 966, fuel_requirement(1969)
    assert_equal 50_346, fuel_requirement(100_756)
  end
end

module_mass = ARGF.readlines
total_fuel_requirement = module_mass.sum { |e| fuel_requirement(e) }
puts total_fuel_requirement

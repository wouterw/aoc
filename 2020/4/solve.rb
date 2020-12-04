# frozen_string_literal: true

def parse_passports(input)
  input.split("\n\n").map do |line|
    line
      .split("\n")
      .flat_map { |e| e.split(' ') }
      .each_with_object({}) do |property, memo|
        k, v = property.split(':')
        memo[k] = v
      end
  end
end

def required_fields?(passport)
  passport['byr'] &&
    passport['iyr'] &&
    passport['eyr'] &&
    passport['hgt'] &&
    passport['hcl'] &&
    passport['ecl'] &&
    passport['pid']
end

def valid_birth_year?(passport)
  byr = passport['byr']&.to_i
  byr && byr >= 1920 && byr <= 2002
end

def valid_issue_year?(passport)
  iyr = passport['iyr']&.to_i
  iyr && iyr >= 2010 && iyr <= 2020
end

def valid_expiration_year?(passport)
  eyr = passport['eyr']&.to_i
  eyr && eyr >= 2020 && eyr <= 2030
end

def valid_height?(passport)
  hgt = passport['hgt']

  return false unless hgt

  value = hgt[..-3].to_i
  unit = hgt[-2..]

  min, max = if unit == 'cm'
               [150, 193]
             else
               [59, 76]
             end

  value >= min && value <= max
end

def valid_hair_color?(passport)
  hcl = passport['hcl']
  hcl && /^#(\d|[a-f]){6}$/.match?(hcl)
end

EYE_COLORS = %w[amb blu brn gry grn hzl oth].freeze

def valid_eye_color?(passport)
  ecl = passport['ecl']
  ecl && EYE_COLORS.include?(ecl)
end

def valid_passport_id?(passport)
  pid = passport['pid']
  pid && /^\d{9}$/.match?(pid)
end

def valid_country_id?(_passport)
  true
end

def valid_passport?(passport, rules = [])
  rules.all? do |rule|
    rule.call(passport)
  end
end

def number_of_valid_passports(input, rules)
  parse_passports(input)
    .map { |passport| valid_passport?(passport, rules) ? 1 : 0 }
    .sum
end

puzzle_input = ARGF.read

demo_input = <<~TXT
  ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
  byr:1937 iyr:2017 cid:147 hgt:183cm

  iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
  hcl:#cfa07d byr:1929

  hcl:#ae17e1 iyr:2013
  eyr:2024
  ecl:brn pid:760753108 byr:1931
  hgt:179cm

  hcl:#cfa07d eyr:2025 pid:166559648
  iyr:2011 ecl:brn hgt:59in
TXT

invalid_passports = <<~TXT
  eyr:1972 cid:100
  hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

  iyr:2019
  hcl:#602927 eyr:1967 hgt:170cm
  ecl:grn pid:012533040 byr:1946

  hcl:dab227 iyr:2012
  ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

  hgt:59cm ecl:zzz
  eyr:2038 hcl:74454a iyr:2023
  pid:3556412378 byr:2007
TXT

valid_passports = <<~TXT
  pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
  hcl:#623a2f

  eyr:2029 ecl:blu cid:129 byr:1989
  iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

  hcl:#888785
  hgt:164cm byr:2001 iyr:2015 cid:88
  pid:545766238 ecl:hzl
  eyr:2022

  iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
TXT

rules_a = [
  method(:required_fields?)
]

rules_b = [
  method(:required_fields?),
  method(:valid_birth_year?),
  method(:valid_issue_year?),
  method(:valid_expiration_year?),
  method(:valid_height?),
  method(:valid_hair_color?),
  method(:valid_eye_color?),
  method(:valid_passport_id?),
  method(:valid_country_id?)
]

puts number_of_valid_passports(demo_input, rules_a)
puts "Puzzel answer a: #{number_of_valid_passports(puzzle_input, rules_a)}"

puts number_of_valid_passports(invalid_passports, rules_b)
puts number_of_valid_passports(valid_passports, rules_b)
puts "Puzzel answer b: #{number_of_valid_passports(puzzle_input, rules_b)}"

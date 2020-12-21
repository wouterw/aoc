# frozen_string_literal: true

def parse_rules(lines)
  lines.each_with_object({}) do |line, memo|
    head, tail = line.split(':')

    memo[head.to_i] = tail.strip
      .split('|')
      .map { |e| e.strip.split.map { |s| eval(s) } }
  end
end

def parse(input)
  rules, messages = input.split("\n\n").map { |part| part.split("\n") }
  [parse_rules(rules), messages]
end

def build_solver(rules)
  Hash.new do |h, k|
    rule = rules[k].map do |subrule|
      subrule
        .map { |e| e.is_a?(Integer) ? h[e] : e }
        .join
    end

    rule = rule.size == 1 ? rule.first : "(#{rule.join('|')})"

    h[k] = rule
  end
end

def part1(input)
  rules, messages = parse(input)

  solver = build_solver(rules)

  regex = Regexp.new(?^ + solver[0] + ?$)

  messages.map { |s| s.match?(regex) ? 1 : 0 }.sum
end

def part2(input)
  rules, messages = parse(input)

  solver = build_solver(rules)
  solver[8] = "(#{solver[42]})+"
  solver[11] = "(?<r>#{solver[42]}\\g<r>?#{solver[31]})"

  regex = Regexp.new(?^ + solver[0] + ?$)

  messages.map { |s| s.match?(regex) ? 1 : 0 }.sum
end

sample_input = <<~TXT
  0: 4 1 5
  1: 2 3 | 3 2
  2: 4 4 | 5 5
  3: 4 5 | 5 4
  4: "a"
  5: "b"

  ababbb
  bababa
  abbbab
  aaabbb
  aaaabbb
TXT

part_2_sample_input = <<~TXT
  42: 9 14 | 10 1
  9: 14 27 | 1 26
  10: 23 14 | 28 1
  1: "a"
  11: 42 31
  5: 1 14 | 15 1
  19: 14 1 | 14 14
  12: 24 14 | 19 1
  16: 15 1 | 14 14
  31: 14 17 | 1 13
  6: 14 14 | 1 14
  2: 1 24 | 14 4
  0: 8 11
  13: 14 3 | 1 12
  15: 1 | 14
  17: 14 2 | 1 7
  23: 25 1 | 22 14
  28: 16 1
  4: 1 1
  20: 14 14 | 1 15
  3: 5 14 | 16 1
  27: 1 6 | 14 18
  14: "b"
  21: 14 1 | 1 14
  25: 1 1 | 1 14
  22: 14 14
  8: 42
  26: 14 22 | 1 20
  18: 15 15
  7: 14 5 | 1 21
  24: 14 1

  abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
  bbabbbbaabaabba
  babbbbaabbbbbabbbbbbaabaaabaaa
  aaabbbbbbaaaabaababaabababbabaaabbababababaaa
  bbbbbbbaaaabbbbaaabbabaaa
  bbbababbbbaaaaaaaabbababaaababaabab
  ababaaaaaabaaab
  ababaaaaabbbaba
  baabbaaaabbaaaababbaababb
  abbbbabbbbaaaababbbbbbaaaababb
  aaaaabbaabaaaaababaa
  aaaabbaaaabbaaa
  aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
  babaaabbbaaabaababbaabababaaab
  aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
TXT

puts part1(sample_input) == 2
puts part2(part_2_sample_input) == 12

puzzle_input = ARGF.read

puts part1(puzzle_input)
puts part2(puzzle_input)

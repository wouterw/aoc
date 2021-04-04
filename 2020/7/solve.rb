# frozen_string_literal: true

rules = File
  .read("./input.txt")
  .split("\n")
  .map { |r| /\A([a-z ]*) bags? contain (.*)\.\z/.match(r)[1..] }
  .to_h
  .transform_values { |v| v.split(',').flat_map { |s| /(([0-9]*) ([a-z ]*)|no other) bags?/.match(s)[2..] }.compact }

class Node
  attr_reader :label, :neighbours

  def initialize(label)
    @label = label
    @neighbours = []
  end

  def <<(node)
    neighbours << node
  end

  def flatten
    neighbours.flat_map(&:flatten).append(label)
  end
end

class Graph
  attr_reader :nodes

  def initialize
    @nodes = Hash.new { |h, k| h[k] = Node.new(k) }
  end

  def [](key)
    nodes[key]
  end
end

# Part one (inverted directed acyclic graph)
graph = Graph.new
rules.each { |key, values| values.each_slice(2) { |_number, value| graph[value] << graph[key] } }

puts graph['shiny gold'].flatten.uniq.count - 1

# Part two (directed acyclic graph)
graph = Graph.new
rules.each { |key, values| values.each_slice(2) { |number, value| number.to_i.times { graph[key] << graph[value] } } }

puts graph['shiny gold'].flatten.count - 1

class Array
  def bubble_sort(&block)
    process = block ? lambda { |index| block.call(self[index], self[index + 1]) } : lambda { |index| self[index] > self[index + 1] }

    self.size.times do |_|
      (self.size - 1).times do |index|
        self[index], self[index + 1] = self[index + 1], self[index] if process.call(index)
      end
    end

    return self
  end
end

p shuffle_numbers = (1..9).to_a.shuffle
# => [6, 4, 1, 2, 8, 3, 7, 5, 9]

p shuffle_numbers.bubble_sort
# => [1, 2, 3, 4, 5, 6, 7, 8, 9]

p shuffle_numbers.bubble_sort { |a, b| b > a }
# => [9, 8, 7, 6, 5, 4, 3, 2, 1]

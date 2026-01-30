#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

std::tuple<size_t, size_t> find_max(const std::vector<unsigned>& digits, size_t start, size_t end) {
    auto max_it = std::max_element(digits.begin() + start, digits.begin() + end);
    return {max_it - digits.begin(), *max_it};
}

size_t calculate_joltage(const std::vector<unsigned>& digits, int block_size) {
    size_t index{0};
    size_t joltage{0};
    for (size_t i = 0; i < block_size; ++i) {
        auto [ind, max] = find_max(digits, index, digits.size() - (block_size - i - 1));
        index = ind + 1;
        joltage = joltage * 10 + max;
    }
    return joltage;
}

size_t part1(const std::vector<std::vector<unsigned>>& arr) {
    return std::accumulate(arr.begin(), arr.end(), 0, [](size_t acc, const auto& digits) {
        return acc + calculate_joltage(digits, 2);
    });
}

size_t part2(const std::vector<std::vector<unsigned>>& arr) {
    return std::accumulate(arr.begin(), arr.end(), 0l, [](size_t acc, const auto& digits) {
        return acc + calculate_joltage(digits, 12);
    });
}

int main() {
    std::ifstream puzzle("puzzle.txt");
    std::string line;
    std::vector<std::vector<unsigned>> arr{};

    while (std::getline(puzzle, line)) {
        std::vector<unsigned> digits{};
        for (int i = 0; i < line.length(); i++) {
            digits.push_back(line[i] - '0');
        }
        arr.push_back(digits);
    }

    std::cout << part1(arr) << std::endl;
    std::cout << part2(arr) << std::endl;
    return 0;
}

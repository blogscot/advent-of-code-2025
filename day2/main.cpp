#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

std::string repeat(std::string_view text, size_t times) {
    std::string result;
    result.reserve(text.length() * times);
    for (size_t i = 0; i < times; i++) {
        result += text;
    }
    return result;
}

bool is_valid(std::string_view text) {
    size_t mid = text.length() / 2;
    return text.substr(0, mid) == text.substr(mid);
}

bool is_valid2(std::string_view text) {
    std::vector<int> primes{2, 3, 5, 7};
    for (auto const& n : primes) {
        size_t len = text.length();
        if (len % n == 0) {
            std::string_view piece = text.substr(0, len / n);
            if (repeat(piece, n).compare(text) == 0) {
                return true;
            }
        }
    }
    return false;
}

using Test = bool (*)(std::string_view);

uint64_t part1(const std::string line, Test test) {
    std::istringstream iss(line);
    std::string currentItem;
    uint64_t total{0};
    while (std::getline(iss, currentItem, ',')) {
        size_t pos = currentItem.find("-");
        uint64_t start = std::stoull(currentItem.substr(0, pos));
        uint64_t end = std::stoull(currentItem.substr(pos + 1));

        for (uint64_t i = start; i <= end; i++) {
            if (test(std::to_string(i))) {
                total += i;
            }
        }
    }
    return total;
}

int main() {
    std::ifstream puzzle("puzzle.txt");
    std::string line;
    std::getline(puzzle, line);
    std::cout << part1(line, is_valid) << std::endl;
    std::cout << part1(line, is_valid2) << std::endl;
    return 0;
}

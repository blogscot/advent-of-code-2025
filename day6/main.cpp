#include <cstddef>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using Matrix = std::vector<std::vector<size_t>>;

Matrix transpose(const Matrix& matrix) {
    Matrix result;
    for (size_t i = 0; i < matrix[0].size(); i++) {
        std::vector<size_t> row;
        for (size_t j = 0; j < matrix.size(); j++) {
            row.push_back(matrix[j][i]);
        }
        result.push_back(row);
    }
    return result;
}

std::vector<size_t> get_positions(std::string_view symbol_line) {
    std::vector<size_t> positions;
    for (size_t i = 0; i < symbol_line.size(); i++) {
        if (symbol_line[i] != ' ') {
            positions.push_back(i);
        }
    }
    positions.push_back(symbol_line.size());
    return positions;
}

size_t get_total(std::string_view symbol_line, const Matrix& matrix) {
    std::vector<size_t> positions = get_positions(symbol_line);
    size_t total{0};
    for (size_t i = 0; i < positions.size() - 1; i++) {
        char symbol = symbol_line[positions[i]];
        if (symbol == '+') {
            total += std::accumulate(matrix[i].begin(), matrix[i].end(), 0ul);
        } else if (symbol == '*') {
            total +=
                std::accumulate(matrix[i].begin(), matrix[i].end(), 1ul, std::multiplies<size_t>());
        }
    }
    return total;
}

size_t part1(std::string_view symbol_line, const std::vector<std::string>& lines) {
    std::vector<size_t> positions = get_positions(symbol_line);

    Matrix matrix;
    for (auto& line : lines) {
        std::vector<size_t> row;
        for (size_t i = 0; i < positions.size() - 1; i++) {
            row.push_back(stoull(line.substr(positions[i], positions[i + 1])));
        }
        matrix.push_back(row);
    }
    Matrix transposed = transpose(matrix);
    return get_total(symbol_line, transposed);
}

size_t part2(std::string_view symbol_line, const std::vector<std::string>& lines) {
    size_t line_len{lines[0].size()};

    size_t index{0};
    std::vector<std::vector<size_t>> matrix;
    std::vector<size_t> numbers;
    for (index = 0; index < line_len; index++) {
        std::vector<char> chars;
        for (auto& line : lines) {
            if (line.at(index) != ' ') {
                chars.push_back(line[index]);
            }
        }
        if (!chars.empty()) {
            std::string s(chars.begin(), chars.end());
            size_t number = stoull(s);
            numbers.push_back(number);
        } else {
            matrix.push_back(numbers);
            numbers.clear();
        }
    }
    matrix.push_back(numbers);
    return get_total(symbol_line, matrix);
}

int main() {
    std::ifstream file("puzzle.txt");
    if (!file) {
        std::cerr << "Error opening file" << std::endl;
        return 1;
    }
    std::string line;
    std::vector<std::string> lines;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }
    std::string symbol_line = lines.back();
    lines.pop_back();

    std::cout << "part1: " << part1(symbol_line, lines) << std::endl;
    std::cout << "part2: " << part2(symbol_line, lines) << std::endl;
    return 0;
}

#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

struct Position {
    int x;
    int y;
};

struct Dimensions {
    size_t height;
    size_t width;
};

int neighbours(std::vector<std::string> const& lines, Dimensions const& dim, Position const&& pos) {
    static const std::vector<Position> deltas = {
        {-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1},
    };
    int count{0};
    std::for_each(deltas.begin(), deltas.end(), [&](Position delta) {
        Position new_pos = {pos.x + delta.x, pos.y + delta.y};
        if (new_pos.x >= 0 && new_pos.x < dim.width && new_pos.y >= 0 && new_pos.y < dim.height) {
            if (lines[new_pos.y][new_pos.x] == '@') {
                count++;
            }
        }
    });
    return count;
}

int part1(std::vector<std::string>& lines, Dimensions const& dim) {
    int count{0};
    for (int i = 0; i < dim.height; i++) {
        for (int j = 0; j < dim.width; j++) {
            int num_neighbours = neighbours(lines, dim, {j, i});
            if (lines[i][j] == '@' && num_neighbours < 4) {
                count++;
            }
        }
    }
    return count;
}

int update(std::vector<std::string>& lines, Dimensions const& dim) {
    const std::vector<std::string> copy = lines;
    int count{0};
    for (int i = 0; i < dim.height; i++) {
        for (int j = 0; j < dim.width; j++) {
            int num_neighbours = neighbours(copy, dim, {j, i});
            if (copy[i][j] == '@' && num_neighbours < 4) {
                count++;
                lines[i][j] = '.';
            } else {
                lines[i][j] = lines[i][j];
            }
        }
    }
    return count;
}

int part2(std::vector<std::string>& lines, Dimensions const& dim) {
    int count = update(lines, dim);
    int prev = count;
    while (true) {
        count += update(lines, dim);
        if (count == prev) {
            break;
        }
        prev = count;
    }
    return count;
}

int main() {
    std::ifstream file("puzzle.txt");
    std::string line;
    std::vector<std::string> lines;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    const size_t height = lines.size();
    const size_t width = lines[0].size();
    const Dimensions dim = {height, width};
    std::cout << part1(lines, dim) << std::endl;
    std::cout << part2(lines, dim) << std::endl;

    return 0;
}

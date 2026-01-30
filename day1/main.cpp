#include <fstream>
#include <iostream>
#include <span>
#include <string>
#include <vector>

struct Turn {
    char direction;
    int distance;
};

int part1(const std::span<Turn>& t) {
    int dial_position{50};
    int num_zeros{0};

    for (auto turn : t) {
        if (turn.direction == 'R') {
            dial_position += turn.distance;
        } else if (turn.direction == 'L') {
            dial_position -= turn.distance;
        }
        dial_position = (dial_position + 100) % 100;
        if (dial_position == 0) {
            num_zeros++;
        }
    }
    return num_zeros;
}

int part2(const std::span<Turn>& t) {
    int dial_position{50};
    int num_zeros{0};

    for (auto turn : t) {
        if (turn.direction == 'R') {
            for (int i = 0; i < turn.distance; i++) {
                dial_position++;
                if (dial_position % 100 == 0) {
                    num_zeros++;
                }
            }
        } else if (turn.direction == 'L') {
            for (int i = 0; i < turn.distance; i++) {
                dial_position--;
                if (dial_position % 100 == 0) {
                    num_zeros++;
                }
            }
        }
        dial_position = (dial_position + 100) % 100;
    }
    return num_zeros;
}

int main() {
    std::ifstream puzzle("puzzle.txt");
    std::vector<Turn> t = {};
    for (std::string line; std::getline(puzzle, line);) {
        char direction = line[0];
        int distance = stoi(line.substr(1));
        t.push_back({line[0], distance});
    }
    std::cout << part1(t) << std::endl;
    std::cout << part2(t) << std::endl;

    return 0;
}

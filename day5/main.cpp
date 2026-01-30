#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

struct Range {
    std::uint64_t start;
    std::uint64_t end;
};

std::vector<Range> pack(const std::vector<Range>& ranges) {
    std::vector<Range> result{ranges[0]};
    for (size_t i = 1; i < ranges.size(); ++i) {
        Range previous = result.back();
        result.pop_back();
        Range current = ranges[i];
        auto [start1, end1] = previous;
        auto [start2, end2] = current;
        // if previous is a subset of current
        if (start1 == start2 && end1 <= end2) {
            result.push_back(current);
        }  // if previous is a superset of current
        else if (start1 <= start2 && end1 >= end2) {
            result.push_back(previous);
        }  // if previous overlaps with current
        else if (end1 >= start2) {
            result.push_back({start1, end2});
        } else {
            result.push_back(previous);
            result.push_back(current);
        }
    }
    return result;
}

int part1(const std::vector<Range>& ranges, const std::vector<std::uint64_t>& ids) {
    int result = 0;
    for (const auto& id : ids) {
        for (const auto [start, end] : ranges) {
            if (id >= start && id <= end) {
                result++;
                break;
            }
        }
    }
    return result;
}

uint64_t part2(std::vector<Range>& ranges) {
    std::sort(ranges.begin(), ranges.end(), [](const Range& lhs, const Range& rhs) {
        return std::tie(lhs.start, lhs.end) <= std::tie(rhs.start, rhs.end);
    });

    uint64_t result = 0;
    for (const auto [start, end] : pack(ranges)) {
        result += end - start + 1;
    }
    return result;
}

int main() {
    std::ifstream input("puzzle.txt");
    std::string line;
    std::vector<Range> ranges;
    std::vector<std::uint64_t> ids;
    while (getline(input, line)) {
        if (line.empty()) {
            break;
        }
        size_t pos = line.find("-");
        Range range{};
        range.start = stoull(line.substr(0, pos));
        range.end = stoull(line.substr(pos + 1));
        ranges.push_back(range);
    }
    while (getline(input, line)) {
        ids.push_back(stoull(line));
    }

    std::cout << part1(ranges, ids) << std::endl;
    std::cout << part2(ranges) << std::endl;

    return 0;
}

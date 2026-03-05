#include <algorithm>
#include <array>
#include <format>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <unordered_map>
#include <vector>

struct PathFinder {
    PathFinder() = delete;
    PathFinder(std::unordered_map<std::string, std::vector<std::string>> devices)
        : devices(devices) {}

    uint32_t search(std::string dest) {
        if (dest == "out") {
            return 1;
        }

        return accumulate(
            devices[dest].begin(), devices[dest].end(), 0,
            [this](uint32_t sum, std::string device) { return sum + search(device); });
    }

    uint64_t cached(std::string const& dest, std::array<std::string, 2> specials) {
        std::unordered_map<std::string, uint64_t> cache;
        std::vector<std::string> visited;
        return search2(dest, specials, visited, cache);
    }

   private:
    std::string build_key(std::string_view dest, std::vector<std::string> const& visited) {
        return std::format("{}{}", dest, std::ranges::fold_left(visited, "", std::plus{}));
    }

    uint64_t search2(std::string const& dest,
                     std::array<std::string, 2> const& specials,
                     std::vector<std::string> const& visited,
                     std::unordered_map<std::string, uint64_t>& cache) {
        std::string key = build_key(dest, visited);
        auto it = cache.find(key);
        if (it != cache.end()) {
            return it->second;
        }
        if (dest == "out") {
            bool found = std::all_of(specials.begin(), specials.end(), [&visited](std::string s) {
                return std::find(visited.begin(), visited.end(), s) != visited.end();
            });
            return found ? 1 : 0;
        }

        uint64_t sum = 0;
        for (auto device : devices[dest]) {
            auto new_visited = visited;
            if (std::find(specials.begin(), specials.end(), device) != specials.end()) {
                new_visited.push_back(device);
            }
            sum += search2(device, specials, new_visited, cache);
        }
        cache[key] = sum;
        return sum;
    }

    std::unordered_map<std::string, std::vector<std::string>> devices;
};

int main() {
    std::ifstream input("puzzle.txt");
    std::string line;
    std::unordered_map<std::string, std::vector<std::string>> devices;

    while (std::getline(input, line)) {
        auto pos = line.find(':');
        std::string name = line.substr(0, pos);
        std::string rest = line.substr(pos + 2);
        std::vector<std::string> dests =
            rest | std::views::split(' ') | std::ranges::to<std::vector<std::string>>();
        devices[name] = dests;
    }

    PathFinder finder(devices);
    std::cout << finder.search("you") << std::endl;

    std::array<std::string, 2> specials = {"dac", "fft"};
    std::cout << finder.cached("svr", specials) << std::endl;
    return 0;
}

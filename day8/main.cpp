#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <vector>

using namespace std;

struct JBox;
using Circuit = std::vector<JBox>;
using Connection = std::tuple<uint64_t, JBox, JBox>;

struct JBox {
    JBox() = default;
    JBox(std::vector<int> const& v) : x(v[0]), y(v[1]), z(v[2]) {}

    uint64_t distance(JBox const& other) {
        const auto dx = x - other.x;
        const auto dy = y - other.y;
        const auto dz = z - other.z;
        return static_cast<uint64_t>(sqrt(dx * dx + dy * dy + dz * dz));
    }

    vector<tuple<uint64_t, JBox>> get_distances(std::vector<JBox> const& points) {
        vector<tuple<uint64_t, JBox>> distances{};
        ranges::transform(points, std::back_inserter(distances),
                          [&](JBox const& p) -> std::tuple<uint64_t, JBox> {
                              return make_tuple(this->distance(p), p);
                          });
        return distances;
    }

    bool operator==(JBox other) { return x == other.x && y == other.y && z == other.z; }

    uint64_t x;
    uint64_t y;
    uint64_t z;
};

template <>
struct std::formatter<JBox> {
    constexpr auto parse(format_parse_context& ctx) { return ctx.begin(); }

    auto format(JBox const& p, format_context& ctx) const {
        return std::format_to(ctx.out(), "[{} {} {}]", p.x, p.y, p.z);
    }
};

vector<Connection> find_connections(vector<JBox> positions) {
    vector<Connection> min_distances{};
    while (positions.size() > 1) {
        JBox position = positions.back();
        positions.pop_back();

        auto distances = position.get_distances(positions);
        ranges::for_each(distances, [&](auto&& distance) {
            auto [min_distance, min_position] = distance;
            min_distances.push_back({min_distance, min_position, position});
        });
    }

    sort(min_distances.begin(), min_distances.end(),
         [](auto&& a, auto&& b) { return std::get<0>(a) < std::get<0>(b); });
    return min_distances;
}

optional<int> get_position(vector<Circuit>& circuits, JBox jbox) {
    auto it = ranges::find_if(circuits, [&](Circuit& circuit) {
        return ranges::find_if(circuit, [&](auto value) { return value == jbox; }) != circuit.end();
    });
    if (it != circuits.end()) {
        return it - circuits.begin();
    } else {
        return nullopt;
    }
}

void search_circuits(vector<Circuit>& circuits, Connection& connection) {
    auto [ignore, jbox1, jbox2] = connection;
    auto jbox1_position = get_position(circuits, jbox1);
    auto jbox2_position = get_position(circuits, jbox2);
    if (!jbox1_position.has_value() && !jbox2_position.has_value()) {
        circuits.push_back({jbox1, jbox2});
    } else
        // One match found, add the other
        if (jbox1_position.has_value() && !jbox2_position.has_value()) {
            circuits[jbox1_position.value()].push_back(jbox2);
        } else if (!jbox1_position.has_value() && jbox2_position.has_value()) {
            circuits[jbox2_position.value()].push_back(jbox1);
        } else
            // Both matches found, merge the circuits if different
            if (jbox1_position.has_value() && jbox2_position.has_value()) {
                if (jbox1_position.value() != jbox2_position.value()) {
                    auto& circuit1 = circuits[jbox1_position.value()];
                    auto& circuit2 = circuits[jbox2_position.value()];
                    circuit1.insert(circuit1.end(), circuit2.begin(), circuit2.end());
                    circuits.erase(circuits.begin() + jbox2_position.value());
                }
            }
}

size_t part1(vector<JBox> positions, size_t num_pairs) {
    vector<Circuit> circuits{};
    auto found = find_connections(positions);
    span<Connection> connection{found};
    auto connections = connection.subspan(0, num_pairs);

    while (!connections.empty()) {
        auto connection = connections.front();
        connections = connections.subspan(1);
        search_circuits(circuits, connection);
    }

    vector<size_t> lengths{};
    for (auto& circuit : circuits) {
        lengths.push_back(circuit.size());
    }
    sort(begin(lengths), end(lengths), greater<size_t>());
    auto top_three = lengths | views::take(3) | views::common;
    return ranges::fold_left(top_three, 1, [](auto acc, auto length) { return acc * length; });
}

size_t max_length(vector<Circuit>& circuits) {
    auto largest =
        ranges::max_element(circuits, [](Circuit& a, Circuit& b) { return a.size() < b.size(); });
    return largest->size();
}

size_t part2(vector<JBox>& positions) {
    vector<Circuit> circuits{};
    auto found = find_connections(positions);
    span<Connection> connections{found};
    while (connections.size() > 0) {
        auto connection = connections.front();
        connections = connections.subspan(1);
        search_circuits(circuits, connection);
        auto max{max_length(circuits)};
        if (max == positions.size()) {
            auto [ignore, jbox1, jbox2] = connection;
            return jbox1.x * jbox2.x;
        }
    }
    return 0;
}

int main() {
    std::ifstream file("puzzle.txt");
    if (!file) {
        std::cerr << "Error opening file" << std::endl;
        return 1;
    }
    std::string line;
    std::vector<JBox> positions;
    while (std::getline(file, line)) {
        auto myVector = std::vector<int>(
            std::from_range, line | std::views::split(',') | std::views::transform([](auto&& s) {
                                 return stoi(std::string(begin(s), end(s)));
                             }));
        positions.emplace_back(myVector);
    }
    file.close();

    size_t num_pairs = 1000;
    println("{}", part1(positions, num_pairs));
    println("{}", part2(positions));
    return 0;
}

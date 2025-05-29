#include "GameState.hpp"
// #include <algorithm>
#include <stdint.h>

#ifdef ARDUINO
#include "Arduino.h"
#define Ard(x) x
#else
#define Ard(x)
#endif

#ifndef ARR_SIZE //(arr)
#define ARR_SIZE(arr) sizeof(arr) / sizeof(arr[0])
#endif

template <typename T> constexpr T&& my_move(T& t) {
    return static_cast<T&&>(t);
}

/// Linear Congruential Generator (LCG)
uint32_t simple_rng(uint32_t& seed) {
    seed = seed * 1664525 + 1013904223;
    return seed;
}

/// Fisher-Yates-Shuffle
template <typename T> void shuffle(T arr[], int size, uint32_t& seed) {
    for (int i = size - 1; i > 0; --i) {
        uint32_t rand_val = simple_rng(seed);
        int j = rand_val % (i + 1);

        T tmp = arr[i];
        arr[i] = arr[j];
        arr[j] = tmp;
    }
}

namespace tetris {

constexpr const Vec2 L[4] = {Vec2{0, 0}, {0, -1}, {0, 1}, {1, 1}};

constexpr const Vec2 R_L[4] = {Vec2{0, 0}, {0, -1}, {0, 1}, {1, -1}};

constexpr const Vec2 I[4] = {Vec2{0, 0}, {0, -1}, {0, 1}, {0, -2}};

constexpr const Vec2 Z[4] = {Vec2{0, 0}, {-1, 0}, {0, -1}, {-1, -1}};

constexpr const Vec2 R_Z[4] = {Vec2{0, 0}, {1, 0}, {0, -1}, {1, -1}};

constexpr const Vec2 B[4] = {Vec2{0, 0}, {-1, -1}, {-1, 0}, {0, -1}};

constexpr const Vec2 T[4] = {Vec2{0, 0}, {0, -1}, {-1, 0}, {1, 0}};

constexpr const Vec2* const Tetrinos[static_cast<size_t>(
    Tetrino::NrOfTetrinos)] = {L, R_L, I, Z, R_Z, B, T};

constexpr const Vec2* const get_tetrino_layout(Tetrino tet) {
    return Tetrinos[static_cast<uint8_t>(tet)];
}
TetrinoBag::TetrinoBag() : bag{} { set_next_batch(); }
TetrinoBag::TetrinoBag(uint32_t seed) : bag{}, seed(seed) { set_next_batch(); }
void TetrinoBag::set_next_batch() {
    for (int i = 0; i < (int)Tetrino::NrOfTetrinos; i++) {
        bag[i] = (Tetrino)i;
    }
    shuffle(bag, (int)Tetrino::NrOfTetrinos, seed);
    index = 0;
}
Tetrino TetrinoBag::next() {
    if (index >= static_cast<uint8_t>(Tetrino::NrOfTetrinos)) {
        set_next_batch();
    }
    return bag[index++];
}
void TetrinoBag::set_seed(uint32_t seed) { this->seed = seed; }

TetrinoBag bag = TetrinoBag();

GameState::GameState(GameState::M&& members) : m(my_move(members)) {}

GameState GameState::create() {
    Tetrino nexts[3];
    set_nexts(nexts);
    return GameState(M{nullopt, nexts});
}

void GameState::set_nexts(Tetrino (&nexts)[3]) {
    Ard(Serial.println("set_nexts"));
    for (int i = 0; i < 3; i++) {
        nexts[i] = bag.next();
    }
}

Tetrino GameState::get_next() {
    auto tet = m.nexts[0];
    for (int i = 0; i < 2; i++) {
        m.nexts[i] = m.nexts[i + 1];
    }
    Ard(Serial.println("set_next"));
    m.nexts[2] = bag.next();
    return tet;
}

bool GameState::is_game_over() {
    for (int i = 0; i < Width; i++) {
        if (m.field[i] != 0) {
            return true;
        }
    }
    return false;
}

bool GameState::can_move_down() {
    if (!m.current.has_value()) {
        return false;
    }
    auto indexes = m.current.value().get_indexes();

    for (size_t i = 0; i < 4; ++i) {
        if (indexes[i] < 0) {
            continue;
        }
        if (m.field[indexes[i] + (Width)] != 0 ||
            (indexes[i] > (Height - 1) * Width &&
             indexes[i] <= Height * Width)) {
            return false;
        }
    }
    return true;
}

bool GameState::can_move_left() {
    if (!m.current.has_value()) {
        return false;
    }

    auto indexes = m.current.value().get_indexes();

    for (size_t i = 0; i < 4; ++i) {
        if (indexes[i] < 0) {
            continue;
        }
        if (indexes[i] % Width == 0 || m.field[indexes[i] - 1] != 0) {
            return false;
        }
    }

    return true;
}

bool GameState::can_move_right() {
    if (!m.current.has_value()) {
        return false;
    }

    auto indexes = m.current.value().get_indexes();

    for (size_t i = 0; i < 4; ++i) {
        if (indexes[i] < 0) {
            continue;
        }
        if (indexes[i] % Width == Width - 1 || m.field[indexes[i] + 1] != 0) {
            return false;
        }
    }

    return true;
}

bool GameState::clear_rows() {
    int writeRow = Height - 1;

    for (int y = Height - 1; y >= 0; --y) {
        bool isFull = true;

        for (int x = 0; x < Width; ++x) {
            if (m.field[y * Width + x] == 0) {
                isFull = false;
                break;
            }
        }

        if (!isFull) {
            // Kopiere Zeile y nach writeRow
            if (writeRow != y) {
                for (int x = 0; x < Width; ++x) {
                    m.field[writeRow * Width + x] = m.field[y * Width + x];
                }
            }
            --writeRow;
        }
    }

    // Setze alle verbleibenden oberen Zeilen auf 0
    for (int y = writeRow; y >= 0; --y) {
        for (int x = 0; x < Width; ++x) {
            m.field[y * Width + x] = 0;
        }
    }
    return writeRow == 0;
}

auto GameState::field_with_floating() -> uint8_t (&)[Width * Height] {
    static uint8_t arr[Width * Height]{0};

    // std::copy(std::begin(m.field), std::end(m.field), std::begin(arr));
    for (std::size_t i = 0; i < sizeof(m.field) / sizeof(m.field[0]); ++i) {
        arr[i] = m.field[i];
    }

    if (!m.current.has_value())
        return arr;
    auto indexes = m.current.value().get_indexes();

    for (size_t i = 0; i < 4; i++) {
        arr[indexes[i]] = 1;
    }

    return arr;
}

bool GameState::game_tick() {
    if (is_game_over()) {
        return is_game_over();
    }
    if (!m.current.has_value()) {
        m.current = TetPos{get_next(), {Width / 2, 0}, Rotation::None};
    }

    if (can_move_down()) {
        m.current->cur_pos.y++;
    } else {
        place(m.current.value());
        m.current = TetPos{get_next(), {Width / 2, 0}, Rotation::None};
    }
    clear_rows();
    return is_game_over();
}

void GameState::place(const TetPos& tet_pos) {
    auto indexes = tet_pos.get_indexes();

    for (size_t i = 0; i < 4; i++) {
        if (indexes[i] < 0) {
            continue;
        }

        m.field[indexes[i]] = 1;
    }
}

void GameState::turn(TetPos& tet, int8_t amount) {
    tet.rot = static_cast<Rotation>(
        positive_mod((static_cast<int8_t>(tet.rot) + amount),
                     static_cast<uint8_t>(Rotation::Max_Rotation)));
}

bool GameState::player_tick(Buttons buttons) {
    if (is_game_over()) {
        return is_game_over();
    }
    if (buttons == Button::Down) {
        if (m.current.has_value() && can_move_down()) {
            m.current->cur_pos.y++;
        }
    }
    if (buttons == Button::Left) {
        if (m.current.has_value() && can_move_left()) {
            m.current->cur_pos.x--;
        }
    }
    if (buttons == Button::Right) {
        if (m.current.has_value() && can_move_right()) {
            m.current->cur_pos.x++;
        }
    }
    if (buttons == Button::Up) {
        while (m.current.has_value() && can_move_down()) {
            m.current->cur_pos.y++;
        }
    }
    if (buttons == Button::Turn_Left) {
        if (m.current.has_value()) {
            turn(m.current.value(), 1);
        }
    }
    if (buttons == Button::Turn_Right) {
        if (m.current.has_value()) {
            turn(m.current.value(), -1);
        }
    }
    if (buttons == Button::Swap) {
    }

    clear_rows();

    return is_game_over();
}

int32_t GameState::to_string(char (&arr)[Width * Height * 2 + 1]) {
    size_t i = 0;
    auto field = field_with_floating();
    for (size_t j = 0; j < Width * Height; j++) {
        arr[i] = field[j] + '0';
        if ((j + 1) % Width == 0) {
            arr[i + 1] = '\n';
        } else {
            arr[i + 1] = ' ';
        }
        i += 2;
    }
    arr[Width * Height * 2] = '\0';

    return i;
}

auto GameState::to_string() -> char (&)[Width * Height * 2 + 1] {
    static char str[Width * Height * 2 + 1]{0};
    to_string(str);
    return str;
}

Vec2 Vec2::operator+(Vec2 rhs) {
    return Vec2{.x = static_cast<int8_t>(this->x + rhs.x),
                .y = static_cast<int8_t>(this->y + rhs.y)};
}

Buttons operator|(Button b1, Button b2) {
    return static_cast<Buttons>(b1) | static_cast<Buttons>(b2);
}
Buttons operator|(Buttons buttons, Button b2) {
    return buttons | static_cast<Buttons>(b2);
}
void operator|(Buttons* buttons, Button b2) {
    *buttons = *buttons | static_cast<Buttons>(b2);
}
Buttons operator&(Button b1, Button b2) {
    return static_cast<Buttons>(b1) & static_cast<Buttons>(b2);
}
Buttons operator&(Buttons buttons, Button b2) {
    return buttons & static_cast<Buttons>(b2);
}

bool operator==(Buttons lhs, Button rhs) {
    return (lhs & static_cast<Buttons>(rhs)) == static_cast<Buttons>(rhs);
}

inline bool operator!=(const Buttons& lhs, const Button& rhs) {
    return !(lhs == rhs);
}

auto TetPos::get_indexes() const -> ptrdiff_t (&)[4] {
    auto pos = Tetrinos[static_cast<size_t>(this->cur)];
    static ptrdiff_t indexes[4]{0};
    static Vec2 new_pos[4]{};

    // std::copy_n(pos, 4, std::begin(new_pos));
    for (int i = 0; i < 4; ++i) {
        new_pos[i] = pos[i];
    }

    for (size_t i = 0; i < ARR_SIZE(new_pos); i++) {
        new_pos[i] = new_pos[i].rotate(rot) + cur_pos;
        indexes[i] = new_pos[i].x + Width * new_pos[i].y;
    }
    return indexes;
}

Vec2 Vec2::rotate(Rotation rot) {
    switch (rot) {
    case Rotation::None:
        return *this;
    case Rotation::Clock:
        return {static_cast<int8_t>(-y), static_cast<int8_t>(x)};
    case Rotation::Mirror:
        return {static_cast<int8_t>(-x), static_cast<int8_t>(-y)};
    case Rotation::CounterClock:
        return {static_cast<int8_t>(y), static_cast<int8_t>(-x)};
    default:
        return *this;
    }
}

} // namespace tetris

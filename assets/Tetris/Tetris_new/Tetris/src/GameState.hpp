#pragma once

#include <stddef.h>
#include <stdint.h>

#include "optional.hpp"
#include "../Config.h"

namespace tetris {
enum class Rotation : uint8_t {
    None = 0,
    CounterClock = 1,
    Mirror = 2,
    Clock = 3,
    Max_Rotation,
};
struct Vec2 {
    int8_t x, y;
    Vec2 operator+(Vec2 rhs);
    Vec2 rotate(Rotation r);
};

enum class Tetrino : uint8_t {
    L = 0,
    R_L = 1,
    I = 2,
    Z = 3,
    R_Z = 4,
    B = 5,
    T = 6,
    NrOfTetrinos,
};
constexpr const Vec2* const get_tetrino_layout(Tetrino tet);

class TetrinoBag {
    Tetrino bag[static_cast<uint8_t>(Tetrino::NrOfTetrinos)];
    uint8_t index = 0;
    uint32_t seed = 12345;

    void set_next_batch();

  public:
    TetrinoBag();
    TetrinoBag(uint32_t seed);
    void set_seed(uint32_t seed);
    Tetrino next();
};
extern TetrinoBag bag;

extern const Vec2 L[4];
extern const Vec2 R_L[4];
extern const Vec2 I[4];
extern const Vec2 Z[4];
extern const Vec2 R_Z[4];
extern const Vec2 B[4];
extern const Vec2 T[4];

extern const Vec2* const Tetrinos[static_cast<size_t>(Tetrino::NrOfTetrinos)];

constexpr uint8_t positive_mod(int value, int mod) {
    return (value % mod + mod) % mod;
}

typedef uint8_t Buttons;

enum class Button : uint8_t {
    Up = 1 << 1,
    Down = 1 << 2,
    Left = 1 << 3,
    Right = 1 << 4,
    Turn_Left = 1 << 5,
    Turn_Right = 1 << 6,
    Swap = 1 << 7,
    Max_Button,
};
Buttons operator|(Button b1, Button b2);
Buttons operator|(Buttons buttons, Button b2);
void operator|(Buttons* buttons, Button b2);
Buttons operator&(Button b1, Button b2);
Buttons operator&(Buttons buttons, Button b2);
bool operator==(Buttons lhs, Button rhs);
inline bool operator!=(const Buttons& lhs, const Button& rhs);

constexpr int shiftRecursive(uint8_t value, int shift = 0) {
    return (value <= 1) ? shift : shiftRecursive(value >> 1, shift + 1);
}

constexpr int getShiftAmount(Button button) {
    return shiftRecursive(static_cast<uint8_t>(button));
}

struct TetPos {
    Tetrino cur;
    Vec2 cur_pos;
    Rotation rot;

    auto get_indexes() const -> ptrdiff_t (&)[4];
};

class GameState {
    struct M {
        uint8_t field[Width * Height];
        Option<TetPos> current;
        Tetrino nexts[3];
        size_t score = 0;

        M(Option<TetPos> current_init, Tetrino nexts_init[3],
          size_t score_init = 0)
            : field{0}, current(current_init),
              nexts{nexts_init[0], nexts_init[1], nexts_init[2]},
              score(score_init) {}
    } m;

    GameState() = delete;
    GameState(M&& members);

    static void set_nexts(Tetrino (&nexts)[3]);
    Tetrino get_next();
    bool can_move_down();
    bool can_move_left();
    bool can_move_right();
    bool clear_rows();
    void place(const TetPos& tet);
    void turn(TetPos& tet, int8_t amount);

    public:
    auto field_with_floating() -> uint8_t (&)[Width * Height];
    ~GameState() = default;
    static GameState create();
    bool is_game_over();
    bool game_tick();
    bool player_tick(Buttons buttons);
    int32_t to_string(char (&arr)[Width * Height * 2 + 1]);
    auto to_string() -> char (&)[Width * Height * 2 + 1];
};
} // namespace tetris

#ifdef TETRIS_IMPLEMENTATION
#include "GameState.cpp"
#endif // TETRIS_IMPLEMENTATION
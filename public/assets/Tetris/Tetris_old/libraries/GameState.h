#ifndef GAMESTATE
#define GAMESTATE

#include <array>
#include <stddef.h>
#include <stdint.h>

struct Vec2 {
  int8_t x, y;
  void to_string(char (&out)[19]);
  void turn_clock();
  void turn_counter_clock();
};

enum class Tetrino : uint8_t {
  L = 0,
  R_L = 1,
  I = 2,
  Z = 3,
  R_Z = 4,
  B = 5,
  T = 6,
};
std::array<Vec2, 4> get_tetrino_layout(Tetrino t);

static constexpr const std::array<Vec2, 4> L = {
    Vec2{0, 0}, {0, -1}, {0, 1}, {1, 1}};

static constexpr const std::array<Vec2, 4> R_L = {
    Vec2{0, 0}, {0, -1}, {0, 1}, {1, -1}};

static constexpr const std::array<Vec2, 4> I = {
    Vec2{0, 0}, {0, -1}, {0, 1}, {0, -2}};

static constexpr const std::array<Vec2, 4> Z = {
    Vec2{0, 0}, {-1, 0}, {0, -1}, {-1, -1}};

static constexpr const std::array<Vec2, 4> R_Z = {
    Vec2{0, 0}, {1, 0}, {0, -1}, {1, -1}};

static constexpr const std::array<Vec2, 4> B = {
    Vec2{0, 0}, {-1, -1}, {-1, 0}, {0, -1}};

static constexpr const std::array<Vec2, 4> T =
    { Vec2{0, 0}, {0, -1}, {-1, 0}, {1, 0} };
static constexpr const std::array<Vec2, 4> Tetrinos[] =
    { L, R_L, I, Z, R_Z, B, T };

enum class Rotation : uint8_t {
  None = 0,
  CounterClock = 1,
  Mirror = 2,
  Clock = 3,
};

struct TetPos {
  Tetrino tet;
  Rotation r;
  Vec2 pos;

  TetPos();
  TetPos(Tetrino tet, uint8_t width);
  void turn();
  void turn(int n);
  void anti_turn();
  std::array<Vec2, 4> get_tetrino_layout();
  std::array<Vec2, 4> get_tetrino_positions();
};

template <size_t H, size_t W> struct GameState {
  std::array<uint8_t, H * W> blocks;
  std::array<Tetrino, 3> nexts;
  TetPos current_tet;
  bool placed;
  const bool debug;

  constexpr static const size_t height = H;
  constexpr static const size_t width = W;
  GameState();
  GameState(bool debug);
  bool tick(uint8_t buttons);
  bool isGameOver();
  Tetrino get_next();
  void init_nexts();
  static Tetrino new_next();
  std::array<uint8_t, W * H> get_curr_field();

private:
  inline void _new_tetrino();
  inline void _turn_tetrino(uint8_t buttons);
  inline void _move(uint8_t buttons);
  void inline _move_down_once();
  void inline _move_left_once();
  void inline _move_right_once();
  inline void _place();
  inline void _remove_rows();
  template <size_t len> bool _collides(std::array<Vec2, len> vecs);
  bool _can_move_down();
  bool _can_move_left();
  bool _can_move_right();
};

enum class Button : uint8_t {
  TurnLeft = 1,
  TurnRight = 2,
  Up = 4,
  Down = 8,
  Left = 16,
  Right = 32,
  Swap = 64,
};
inline bool operator==(const uint8_t &lhs, const Button &rhs);
inline bool operator!=(const uint8_t &lhs, const Button &rhs);
uint8_t operator|(Button b1, Button b2);
uint8_t operator|(uint8_t buttons, Button b2);
void operator|(uint8_t *buttons, Button b2);
uint8_t operator&(Button b1, Button b2);
uint8_t operator&(uint8_t buttons, Button b);

int random_int(int min, int max);
int random_int(int max);

#ifdef GAMESTATE_IMPLEMENTATION
#include "GameState.cpp"
#endif

#endif
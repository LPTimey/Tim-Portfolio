#include "GameState.h"

#include <array>
#include <cassert>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

template <size_t H, size_t W>
GameState<H, W>::GameState()
    : blocks{}, nexts{}, current_tet{}, placed{true}, debug{false} {
  init_nexts();
  blocks.fill(0);
}

template <size_t H, size_t W>
GameState<H, W>::GameState(bool debug)
    : blocks{}, nexts{}, current_tet{}, placed{true}, debug{debug} {
  init_nexts();
  blocks.fill(0);
}

template <size_t H, size_t W> inline void GameState<H, W>::_new_tetrino() {
  current_tet = TetPos(get_next(), W);
}
template <size_t H, size_t W>
inline void GameState<H, W>::_turn_tetrino(uint8_t buttons) {
  if (buttons == Button::TurnLeft) {
    current_tet.turn();
  } else if (buttons == Button::TurnRight) {
    current_tet.anti_turn();
  }
}

template <size_t H, size_t W>
template <size_t len>
bool GameState<H, W>::_collides(std::array<Vec2, len> vecs) {
  for (auto vec : vecs) {
    if (vec.x < 0 || vec.y < 0) {
      continue;
    }
    if (vec.y == H) {
      return true;
    }
    if (blocks[unsigned(vec.x + (vec.y * signed(W)))] == 1) {
      return true;
    }
  }
  return false;
}

template <size_t H, size_t W>
inline void GameState<H, W>::_move(uint8_t buttons) {
  if (buttons == Button::Left) {
    if (_can_move_left()) {
      _move_left_once();
    }
  } else if (buttons == Button::Right) {
    if (_can_move_right()) {
      _move_right_once();
    }
  }
  if (buttons == Button::Up) {
    // place
    while (_can_move_down()) {
      _move_down_once();
    }
    _place();
  } else {
    // move
    if (_can_move_down())
      _move_down_once();
    // place?
    else {
      _place();
      // return if already place
      return;
    }

    if (buttons == Button::Down) {
      // move
      if (_can_move_down())
        _move_down_once();
      // place?
      else
        _place();
    }
  }
}

template <size_t H, size_t W> bool GameState<H, W>::_can_move_down() {
  auto pos = current_tet.get_tetrino_positions();
  for (auto &vec : pos) {
    vec.y += 1;
  }
  return !_collides(pos);
}

template <size_t H, size_t W> bool GameState<H, W>::_can_move_left() {
  auto pos = current_tet.get_tetrino_positions();
  for (auto &vec : pos) {
    vec.x -= 1;
    if (vec.x < 0 || vec.x >= signed(W)) {
      return false;
    }
  }
  return !_collides(pos);
}
template <size_t H, size_t W> bool GameState<H, W>::_can_move_right() {
  auto pos = current_tet.get_tetrino_positions();
  for (auto &vec : pos) {
    vec.x += 1;
    if (vec.x < 0 || vec.x >= signed(W)) {
      return false;
    }
  }
  return !_collides(pos);
}
template <size_t H, size_t W> inline void GameState<H, W>::_move_down_once() {
  current_tet.pos.y += 1;
}
template <size_t H, size_t W> inline void GameState<H, W>::_move_left_once() {
  current_tet.pos.x -= 1;
}
template <size_t H, size_t W> inline void GameState<H, W>::_move_right_once() {
  current_tet.pos.x += 1;
}
template <size_t H, size_t W> inline void GameState<H, W>::_place() {
  auto pos = current_tet.get_tetrino_positions();
  for (auto vec : pos) {
    if (vec.x > signed(W) || vec.x < 0 || vec.y > signed(H) || vec.y < 0) {
      continue;
    }
    blocks[unsigned(vec.x + (vec.y * signed(W)))] = 1;
    if (debug)
      printf("placed at ( %i, %i)\n", vec.x, vec.y);
  }
  placed = true;
}

template <size_t H, size_t W> inline void GameState<H, W>::_remove_rows() {
  if (debug)
    printf("removing rows?\n");
  // Go through all rows
  for (size_t curr_y = 1; curr_y < H; curr_y++) {

    bool row_is_complete = true;
    for (size_t curr_x = 0; curr_x < W; curr_x++) {
      if (blocks[curr_x + curr_y * W] == 0) {
        row_is_complete = false;
      }
      if (!row_is_complete) {
        break;
      }
    }

    if (row_is_complete) {
      if (debug)
        printf("removing row %i\n", unsigned(curr_y));
      // move all rows one down
      for (size_t i = curr_y; i != 0; i--) {
        // make row be the same as above (last row should be 0?)
        for (size_t j = 0; j < W; j++) {
          blocks[j + i * W] = blocks[j + (i - 1) * W];
        }
      }
      // just to be sure
      for (size_t j = 0; j < W; j++) {
        blocks[j] = 0;
      }
    }
  }
}

/// TODO: impl a hold watcher (ie was button pressed last frame)
template <size_t H, size_t W> bool GameState<H, W>::tick(uint8_t buttons) {
  // Serial.print("Buttons: ");
  // Serial.println(buttons);
  // Serial.print("Matrix: ");
  // for (size_t i = 0; i < H * W; i++) {
  //   if (i % W == 0)
  //     Serial.println();
  //   Serial.print(blocks[i]);
  //   Serial.print(" ");
  // }
  // Serial.println();

  if (isGameOver()) {
    return true;
  }

  if (placed) {
    _new_tetrino();
    placed = false;
  }

  _turn_tetrino(buttons);

  // check collision after turning
  auto pos = current_tet.get_tetrino_positions();
  bool collision = _collides(pos);

  // move to nearest empty? (slide)
  if (!collision) {
    // TODO: move away from turn-collision
  }

  // move Tetrino?
  _move(buttons);

  // remove row?
  if (placed) {
    _remove_rows();
  }
  return isGameOver();
}

template <size_t H, size_t W> bool GameState<H, W>::isGameOver() {
  for (size_t i = 0; i < W; i++) {
    if (blocks[i]) {
      return true;
    }
  }
  return false;
}
template <size_t H, size_t W> void GameState<H, W>::init_nexts() {
  for (Tetrino &t : nexts) {
    t = new_next();
  }
}
template <size_t H, size_t W> Tetrino GameState<H, W>::get_next() {
  auto first = nexts[0];
  nexts[0] = nexts[1];
  nexts[1] = nexts[2];
  nexts[2] = new_next();
  return first;
}

template <size_t H, size_t W> Tetrino GameState<H, W>::new_next() {

  return static_cast<Tetrino>(random_int(7));
}

template <size_t H, size_t W>
std::array<uint8_t, W * H> GameState<H, W>::get_curr_field() {
  auto field = this->blocks;
  if (!placed) {
    for (auto vec : this->current_tet.get_tetrino_positions()) {
      if (vec.x < 0 || vec.y < 0) {
        continue;
      }
      field[unsigned(vec.x + (vec.y * signed(W)))] = 1;
    }
  }
  return field;
}

std::array<Vec2, 4> get_tetrino_layout(Tetrino t) {
  return Tetrinos[static_cast<uint8_t>(t)];
}

TetPos::TetPos() : tet{}, r{Rotation::None}, pos{} {}

TetPos::TetPos(Tetrino t, uint8_t width) {
  this->tet = t;
  this->r = Rotation::None;
  this->pos = {static_cast<int8_t>((width / 2)), 0};
}

void TetPos::turn(int n) {
  uint8_t rot = static_cast<uint8_t>(r);

  uint8_t new_rot =
      rot + n >= 0 ? unsigned((rot + n) % 4) : unsigned((rot + 4 + n) % 4);
  this->r = static_cast<Rotation>(new_rot);
}
void TetPos::turn() { turn(1); }
void TetPos::anti_turn() { turn(-1); }

std::array<Vec2, 4> TetPos::get_tetrino_layout() {
  auto vecs = ::get_tetrino_layout(this->tet);
  for (auto &vec : vecs) {
    for (size_t i = 0; i < static_cast<uint8_t>(r); i++) {
      vec.turn_counter_clock();
    }
  }
  return vecs;
}
std::array<Vec2, 4> TetPos::get_tetrino_positions() {
  auto arr = get_tetrino_layout();
  for (Vec2 &vec : arr) {
    vec.x += pos.x;
    vec.y += pos.y;
  }
  return arr;
}

uint8_t operator|(Button b1, Button b2) {
  return static_cast<uint8_t>(b1) | static_cast<uint8_t>(b2);
}
uint8_t operator|(uint8_t buttons, Button b2) {
  return buttons | static_cast<uint8_t>(b2);
}
void operator|(uint8_t *buttons, Button b2) {
  *buttons = *buttons | static_cast<uint8_t>(b2);
}
uint8_t operator&(Button b1, Button b2) {
  return static_cast<uint8_t>(b1) & static_cast<uint8_t>(b2);
}
uint8_t operator&(uint8_t buttons, Button b2) {
  return buttons & static_cast<uint8_t>(b2);
}

inline bool operator==(const uint8_t &lhs, const Button &rhs) {
  return (lhs & rhs) == static_cast<uint8_t>(rhs);
}
inline bool operator!=(const uint8_t &lhs, const Button &rhs) {
  return !(lhs == rhs);
}
void Vec2::to_string(char (&out)[19]) {
  sprintf(out, "Vec2( %i, %i )", this->x, this->y);
}
void Vec2::turn_counter_clock() {
  auto temp = this->x;
  this->x = this->y;
  this->y = -temp;
}
void Vec2::turn_clock() {
  auto temp = this->x;
  this->x = -this->y;
  this->y = temp;
}

int random_int(int min, int max) {
  assert(max > min);
  return min + random_int(max - min);
}
int random_int(int max) {
  int limit;
  int r;

  limit = RAND_MAX - (RAND_MAX % max);

  while ((r = rand()) >= limit)
    ;

  return r % max;
}

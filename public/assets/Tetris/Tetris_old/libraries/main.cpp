#include <iostream>
#define GAMESTATE_IMPLEMENTATION
#include "GameState.h"
#include <string>

using std::cout, std::endl;

int main() {
  // cout << "Hello World!" << endl;
  const auto height = 10;
  const auto width = 7;
  const bool debug = false;

  GameState<height, width> game{debug};
  uint8_t buttons = 0;

  auto get_input = [](uint8_t *buttons) {
    *buttons = 0;
    cout << "\n( :nothing; s:down, w:drop, a:left, d:right; q:turn left, "
            "e:turn right)\nEnter to Confirm\n";
    std::string chars;
    std::getline(std::cin, chars);
    // cout <<"input: "<< chars<<"\n";
    for (auto chr : chars) {
      switch (chr) {
      case 'w': {
        buttons | Button::Up;
        break;
      }
      case 'a': {
        buttons | Button::Left;
        break;
      }
      case 's': {
        buttons | Button::Down;
        break;
      }
      case 'd': {
        buttons | Button::Right;
        break;
      }
      case 'q': {
        buttons | Button::TurnLeft;
        break;
      }
      case 'e': {
        buttons | Button::TurnRight;
        break;
      }
      // case 'r': {
      //   buttons | Button::Swap;
      //   break;
      // }
      default: {
        break;
      }
      }
    }
    // cout << unsigned(*buttons);
    return true;
  };

  while (!game.isGameOver() && get_input(&buttons)) {
    game.tick(buttons);
    auto field = game.get_curr_field();
    for (size_t y = 0; y < height; y++) {
      for (size_t x = 0; x < width; x++) {
        size_t i = x + y * width;
        // printf("%li ", field[i]);
        cout << unsigned(field[i]) << " ";
      }
      if (debug) {
        cout << "\t";
        for (size_t x = 0; x < width; x++) {
          size_t i = x + y * width;
          cout << unsigned(game.blocks[i]) << " ";
        }
      }
      cout << "\n";
    }
  }
  cout << endl;

  return 0;
}
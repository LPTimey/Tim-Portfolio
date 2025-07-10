#ifndef ARDUINO
#include "../Config.h"

#include <iostream>
#include <string>
// #include <stdio.h>

#define TETRIS_IMPLEMENTATION
#include "GameState.hpp"

#ifndef ARR_SIZE //(arr)
#define ARR_SIZE(arr) sizeof(arr) / sizeof(arr[0])
#endif

bool get_input(tetris::Buttons* buttons) {
    *buttons = 0;
    std::cout << "\n( :nothing; s:down, w:drop, a:left, d:right; q:turn left, "
                 "e:turn right)\nEnter to Confirm\n";
    std::string chars;
    std::getline(std::cin, chars);
    // cout <<"input: "<< chars<<"\n";
    for (auto chr : chars) {
        switch (chr) {
        case 'w': {
            buttons | tetris::Button::Up;
            break;
        }
        case 'a': {
            buttons | tetris::Button::Left;
            break;
        }
        case 's': {
            buttons | tetris::Button::Down;
            break;
        }
        case 'd': {
            buttons | tetris::Button::Right;
            break;
        }
        case 'q': {
            buttons | tetris::Button::Turn_Left;
            break;
        }
        case 'e': {
            buttons | tetris::Button::Turn_Right;
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
}

int main(void) {
    // printf("Hello World\n");
    std::cout << "Hello World" << std::endl;
    char string[(Height * Width * 2) + 1]{0};
    tetris::Buttons buttons = 0;

    tetris::GameState game = tetris::GameState::create();

    // while (!game.game_tick()) {
    //     game.to_string(string);
    //     std::cout << string << "\n\n";
    // }
    while (get_input(&buttons) && !game.player_tick(buttons) &&
           !game.game_tick()) {
        game.to_string(string);
        std::cout << string << "\n\n";
    }
    std::cout << std::endl;

    // printf("%s", string);
    return 0;
}
#endif
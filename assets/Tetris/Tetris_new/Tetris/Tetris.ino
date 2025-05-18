#include "Config.h"
// #define TETRIS_IMPLEMENTATION
#include "src/GameState.hpp"

#include "Arduino_LED_Matrix.h"
ArduinoLEDMatrix matrix;

tetris::GameState* game() {
    static auto game = tetris::GameState::create();
    return &game;
}

#define TurnLeftPin 7
#define MoveLeftPin 6
#define MoveDownPin 5
#define DropDownPin 4
#define MoveRightPin 3
#define TurnRightPin 2

uint8_t rotated_buffer[Width * Height] = {0};
unsigned long lastPlayerTick = 0;
unsigned long lastGameTick = 0;
// if (millis() > lastTick + interval) { ... }  // kann bei Overflow falsch sein
// if (millis() - lastTick >= interval) { ... } // funktioniert korrekt, auch
// bei Overflow

const unsigned long playerTickInterval = 1000 / 60; // ~16 ms (60 FPS)
unsigned long gameTickInterval = 500; // z. B. 500 ms (fällt jede halbe Sekunde)

void panic() {
    while (true) {
    }
}

tetris::Buttons get_input() {
    tetris::Buttons buttons = 0;
    buttons = buttons | ((uint8_t)digitalRead(TurnLeftPin)
                         << tetris::getShiftAmount(tetris::Button::Turn_Left));
    buttons = buttons | ((uint8_t)digitalRead(MoveLeftPin)
                         << tetris::getShiftAmount(tetris::Button::Left));
    buttons = buttons | ((uint8_t)digitalRead(MoveDownPin)
                         << tetris::getShiftAmount(tetris::Button::Down));
    buttons = buttons | ((uint8_t)digitalRead(DropDownPin)
                         << tetris::getShiftAmount(tetris::Button::Up));
    buttons = buttons | ((uint8_t)digitalRead(MoveRightPin)
                         << tetris::getShiftAmount(tetris::Button::Right));
    buttons = buttons | ((uint8_t)digitalRead(TurnRightPin)
                         << tetris::getShiftAmount(tetris::Button::Turn_Right));
    return buttons;
}

template <typename T>
void rotate(T* source, T* destination, size_t height, size_t width) {
    for (size_t row = 0; row < height; ++row) {
        for (size_t col = 0; col < width; ++col) {

            // 90° CW  col * height + (height - 1 - row)
            // destination[col * height + (height - 1 - row)] = source[row *
            // width + col];

            // 90° CCW (width - 1 - col) * height + row
            destination[(width - 1 - col) * height + row] =
                source[row * width + col];

            // 180°    (height - 1 - row) * width + (width - 1 - col)
            // destination[(height - 1 - row) * width + (width - 1 - col)] =
            // source[row * width + col];

            // Transpose   col * height + row (kein Flip)
            // destination[col * height + row] = source[row * width + col];
        }
    }
}

void setup() {
    Serial.begin(115200);
    // Lese "Rauschen" von einem unverbundenen Analog-Pin
    long seed = 0;
    for (int i = 0; i < 16; i++) {
        seed ^= analogRead(A0) << (i % 8); // Shift für Varianz
        delay(10);                         // Etwas Zeit für Schwankung
    }
    seed ^= analogRead(A1);
    seed ^= analogRead(A2);
    seed ^= millis() << 3;
    randomSeed(seed);
    Serial.println(seed);
    tetris::bag = tetris::TetrinoBag(seed);

    game();

    // for (int i = 0; i<10; i++) {
    //     Serial.print((uint8_t)tetris::bag.next());
    //     Serial.print(", ");
    // }
    // Serial.println();

    pinMode(TurnLeftPin, INPUT);
    pinMode(MoveLeftPin, INPUT);
    pinMode(MoveDownPin, INPUT);
    pinMode(DropDownPin, INPUT);
    pinMode(MoveRightPin, INPUT);
    pinMode(TurnRightPin, INPUT);

    matrix.begin();
}

void loop() {
    // return;
    unsigned long now = millis();

    // TODO add input buffer
    tetris::Buttons input = get_input();

    // PLAYER TICK (z. B. 60 FPS)
    if (now - lastPlayerTick >= playerTickInterval && !game()->is_game_over()) {
        lastPlayerTick = now;
        game()->player_tick(input);
        rotate(game()->field_with_floating(), rotated_buffer, Height, Width);
        matrix.loadPixels(rotated_buffer, Height * Width);
    }

    // GAME TICK (z. B. 1x pro 500ms)
    if (now - lastGameTick >= gameTickInterval && !game()->is_game_over()) {
        lastGameTick = now;
        game()->game_tick();
        rotate(game()->field_with_floating(), rotated_buffer, Height, Width);
        matrix.loadPixels(rotated_buffer, Height * Width);
        for (unsigned int i = 0; i < Height; i++) {
            for (unsigned int j = 0; j < Width; j++) {
                Serial.print(game()->field_with_floating()[j + i * Width]);
                Serial.print(", ");
            }
            Serial.println();
        }
        Serial.println();
    }

    if (game()->is_game_over()) {
        uint8_t frame[8][12] = {{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
                                {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}};
        matrix.renderBitmap(frame, 8, 12);
        panic();
    }
}


// #region include_dependencies
#include <Arduino.h>
#include "Arduino_LED_Matrix.h"

#define DONT_USE_CPP_STL
#define GAMESTATE_IMPLEMENTATION
#include "libraries/GameState.h"
// #endregion include_dependencies

// #region setup_globals
// #region setup_game
#define WIDTH 12
#define HEIGHT 8
GameState<HEIGHT, WIDTH> game = {};
// #endregion setup_game

ArduinoLEDMatrix matrix;
uint32_t StartTime;
bool game_over = false;
uint8_t buttons = 0;

// #region setup_buttons
int btnMoveLeftPin = 7;
int btnMoveLeftPressed;

int btnTurnLeftPin = 6;
int btnTurnLeftPressed;

int btnDropPin = 5;
int btnDropPressed;

int btnBitDownPin = 4;
int btnBitDownPressed;

int btnTurnRightPin = 3;
int btnTurnRightPressed;

int btnMoveRightPin = 2;
int btnMoveRightPressed;

// #endregion setup_buttons
// #endregion setup_globals

uint8_t get_button_data() { 
  // reset buttons
  uint8_t button_data = 0;

  // read buttons
  btnTurnLeftPressed = digitalRead(btnTurnLeftPin);
  btnMoveLeftPressed = digitalRead(btnMoveLeftPin);
  btnDropPressed = digitalRead(btnDropPin);
  btnBitDownPressed = digitalRead(btnBitDownPin);
  btnMoveRightPressed = digitalRead(btnMoveRightPin);
  btnTurnRightPressed = digitalRead(btnTurnRightPin);

  // set buttons
  if (btnTurnLeftPressed == HIGH) {
    button_data = button_data | Button::TurnLeft;
  }
  if (btnMoveLeftPressed == HIGH) {
    &button_data | Button::Left;
  }
  if (btnDropPressed == HIGH) {
    &button_data | Button::Up;
  }
  if (btnBitDownPressed == HIGH) {
    &button_data | Button::Down;
  }
  if (btnMoveRightPressed == HIGH) {
    &button_data | Button::Right;
  }
  if (btnTurnRightPressed == HIGH) {
    &button_data | Button::TurnRight;
  }

  return button_data;
};

void setup() {
  pinMode(btnTurnLeftPin, INPUT);
  pinMode(btnMoveLeftPin, INPUT);
  pinMode(btnDropPin, INPUT);
  pinMode(btnBitDownPin, INPUT);
  pinMode(btnMoveRightPin, INPUT);
  pinMode(btnTurnRightPin, INPUT);

  Serial.begin(115200);
  matrix.begin();
  StartTime = millis();
}

void loop() {
  buttons = get_button_data();
  if(!game_over && TimePeriodIsOver(StartTime, 750)){
    Serial.print("buttons: ");
    Serial.println(buttons);
    game_over = game.tick(buttons);
    matrix.loadPixels(&(game.get_curr_field()[0]), WIDTH * HEIGHT);
    if (game_over) {
      // game = &GameState<HEIGHT, WIDTH>();
    }
  }
  delay(50);
}

/** easy to use helper-function for non-blocking timing
 * @param startOfPeriod - starttime in ms
 * @param TimePersiod - time in ms
 */ 
boolean TimePeriodIsOver (uint32_t &startOfPeriod, uint32_t TimePeriod) {
  uint32_t currentMillis  = millis();
  if ( currentMillis - startOfPeriod >= TimePeriod ) {
    // more time than TimePeriod has elapsed since last time if-condition was true
    startOfPeriod = currentMillis; // a new period starts right here so set new starttime
    return true;
  }
  else return false;            // actual TimePeriod is NOT yet over
}
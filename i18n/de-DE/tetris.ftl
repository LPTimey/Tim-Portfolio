description = Spiele-Entwicklung auf embedded systems mit manueller Input Hardware Eingabe und simpler LED Ausgabe.
content = 
    Im Rahmen eines Hackathons an der Hochschule habe ich eine minimalistische, 
    aber voll spielbare Version von Tetris für den Arduino entwickelt. 
    Die erste Version entstand auf dem Arduino Uno R4, später folgte die Portierung auf den Uno Rev3, 
    was einige technische Änderungen nötig machte.
hardware = Hardware
preparation = Vorbereitung
hardware-prep = 
    TODO:<br/>
    Die verwendete Hardware bestand auf folgenden Teilen:
small = klein
optional = Optional

result = Ergebnis
result-coarse =
    Am Ende des Hackathons hatte ich ein funktionsfähiges Tetris Spiel auf der eingebauten Matrix
    des Arduino R4, jedoch nicht ohne Abzüge. Leider waren weder die mitgebrachte noch die gestellte
    LED-Matrix mit dem neuen R4 kompatibel und ich musste deswegen bei der eingebauten Matrix bleiben.
    Auch ein mit Matrix kompatibler Arduino R3 wurde gestellt, doch dieser war leider mit meinem,
    auf dem R4 getestetem, Source-Code nicht kompatibel (z.B wegen std::array).

breadboard = Steckbrett
resistors = Widerstände
matrix = LED Matrix
connectors = Stecker, Kabel und mehr

lessons-learned = Lessons Learned
lessons-learned-text =
    Ohne Betriebssystem steht deutlich weniger Unterstützung zur Verfügung.
    Die Kompatibilität mit der STL variiert stark.
    Die Arbeit mit Hardware erfordert oft Fingerspitzengefühl; Kurzschlüsse oder Verwirrung sind schnell möglich.

lessons-learned = Lessons Learned
lessons-learned-text =
    Ohne Betriebssystem steht deutlich weniger Unterstützung zur Verfügung.
    Die Kompatibilität mit der STL variiert stark.
    Die Arbeit mit Hardware erfordert oft Fingerspitzengefühl; Kurzschlüsse oder Verwirrung sind schnell möglich.

follow-up = Follow-up work
follow-up-text =
    Nach dem Hackathon habe ich die Tetris-Bibliothek vollständig überarbeitet, sodass sie nun
    auch auf Rev3-Arduinos funktioniert. Dabei ersetzte ich die bisher verwendete C++-STL durch
    klassischen C/C++-Code. Beispielsweise wurde std::array<T,N> durch einfache Array-Deklarationen
    wie type_T name[N] ersetzt.

    Zudem habe ich den bisherigen GameLoop, der über die Funktion tick() lief, in zwei getrennte
    Funktionen aufgeteilt:
    bool game_tick(); und bool player_tick(Buttons buttons);. Dadurch konnte ich die
    Eingabeverarbeitung vom automatischen Fallverhalten entkoppeln, was das Spiel deutlich
    reaktionsschneller macht.
    Eingaben können nun jederzeit erkannt werden, nicht nur während eines Ticks.

how-run = Anleitung
requirements = Voraussetzungen
pc = PC
step-by-step = Schritt für Schritt Anleitung
src = Quellcode
download = Download
listed-hardware = Aufgelistete oder gleichwertige Hardware
hardware-build = Hardware wie oben anbauen
de-zip = Projekt-ZIP-Datei entpacken
cd = In den Projektordner wechseln
open-ide = Arduino IDE darin öffnen
build-ino = Tetris.ino bauen und auf das Arduino-Board flash-en
compiler = C/C++-Compiler (z.B: clang++)
build-nob = nob.c (Buildsystem) zu einer ausführbaren Datei kompilieren
unix-like = Unix artig
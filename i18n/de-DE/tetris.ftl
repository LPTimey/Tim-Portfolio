description = Spiele-Entwicklung auf embedded systems mit manueller Input Hardware Eingabe und simpler LED Ausgabe.
content = 
    Im Rahmen eines Hackathons an der Hochschule habe ich eine minimalistische, 
    aber voll spielbare Version von Tetris für den Arduino entwickelt. 
    Die erste Version entstand auf dem Arduino Uno R4, später folgte die Portierung auf den Uno Rev3, 
    was einige technische Änderungen nötig machte.
hardware = Hardware
preparation = Vorbereitung

hardware-prep = 
    Vor dem Hackathon habe ich nach Hardware gesucht und diese anschließend auch gekauft.
    Ziel war es, Tetris auf der im R4 eingebauten Matrix zu testen und dann auf einer LED-Matrix auszugeben.
    Leider war mir nicht bewusst, dass der R4 nicht auf demselben Chipdesign
    basiert, wie die vorherigen Arduinos, und daher nicht mit der Waveshare-Matrix kompatibel ist.
    Außerdem stellte sich heraus, dass der gegebene R3 zu schwach ist.
hardware-list-h = Die verwendete Hardware bestand auf folgenden Teilen:
UNO-R4-link = Shop
UNO-R3 = Arduino UNO R3 (gegeben vom Hackathon & nicht notwendig)
small = klein
big = großes
optional = Optional
resistors = Widerstände
cables = Kabel
buttons = Knöpfe
wave-screen = WaveShare screen (Fehlentscheidung)

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

game-state-text = 
    Für den Hackathon, habe ich eine Tetris Bibliothek in C++ entwickelt und als Nachbereitung auf R3 erweitert. 
    Diese basiert auf GameState welches alle wichtigen Daten einer runde speichert und Tick-Methoden 
    bereitstellt um das spiel zu treiben. Es stellt auch eine Methode bereit, um das aktuelle 
    Feld entweder als String oder Liste zu bekommen und es anzeigen zu können.
input-text = 
    Um die Nutzereingabe zu lesen werden 2 Typen exportiert: Buttons und Button. Button ist ein Enum 
    welches alle Knöpfe auflistet und je einem Bit in einem Byte zuordnet, sodass alle möglichen 
    Eingaben gleichzeitig und speichersparend verarbeitet werden können, da sie nun in einen 
    Byte (Buttons) passen. Das ermöglicht schnelle Abfragen und kompakte Logik. Man kann sich dieses 
    Flaggen-System vorstellen wie 8 boolesche Werte in einer Variable. Man kann diese Werte mit 
    Bit shifts ( << ) und Bit-Oder ( | ) setzen und mit Bit shifts und Bit-Und ( & ) lesen.
more-on = Mehr Dazu
tet-array-text = 
    Die einzelnen Tetrominos sind in einem Enum als Indexe zu einem Array, welcher Postions-Matrizen 
    der Tetrominos speichert. Um auch Position & Rotation zu speicher wird TetPos benutzt.
tet-bag-text = 
    Die Erscheinungsraten der Tetrominos werden mit Hilfe eines Taschensystems generiert. 
    Diese Tasche generiert alle Tetrominos und randomisiert ihre Order um zu garantieren, 
    sodass es keine Folge "Ziehungen" gibt ein welcher eine art 
    Tetromino öfter als 2 mal oder gar nicht vorkommt. 

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
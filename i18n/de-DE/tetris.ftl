description = Spiele-Entwicklung auf embedded systems mit manueller Input Hardware Eingabe und simpler LED Ausgabe.
content = 
    Im Rahmen eines Hackathons an der Hochschule habe ich eine minimalistische, 
    aber voll spielbare Version von Tetris für den Arduino entwickelt. 
    Die erste Version entstand auf dem Arduino Uno R4, später folgte die Portierung auf den Uno Rev3, 
    was einige technische Änderungen nötig machte.
hardware = Hardware
preparation = Vorbereitung
hardware-prep = 
    TODO:
result = Ergebnis
result-coarse =
    Am Ende des Hackathons hatte ich ein funktionsfähiges Tetris Spiel auf der eingebauten Matrix
    des Arduino R4, jedoch nicht ohne Abzüge. Leider waren weder die mitgebrachte noch die gestellte
    LED-Matrix mit dem neuen R4 kompatibel und ich musste deswegen bei der eingebauten Matrix bleiben.
    Auch ein mit Matrix kompatibler Arduino R3 wurde gestellt, doch dieser war leider mit meinem,
    auf dem R4 getestetem, Source-Code nicht kompatibel (z.B wegen std::array).
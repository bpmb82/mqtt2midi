#include <LiquidCrystal_I2C.h>

// set the LCD number of columns and rows
int lcdColumns = 16;
int lcdRows = 2;

int buttonPin = 0;
int buttonState = 0;
int lastButtonState = 0;
int currentCharIndex = 0;

LiquidCrystal_I2C lcd(0x27, lcdColumns, lcdRows);  

String messageStatic = "Static message";
String messageToScroll = "This is a scrolling message with more than 16 characters";

void square_wave() {
  lcd.clear();

  byte image07[8] = {B01111, B01001, B01001, B11001, B00001, B00001, B00001, B00001};
  byte image08[8] = {B00000, B00000, B00000, B01100, B01000, B01000, B01000, B11000};

  lcd.createChar(0, image07);
  lcd.createChar(1, image08);

  lcd.setCursor(6, 0);
  lcd.write(byte(0));
  lcd.setCursor(7, 0);
  lcd.write(byte(1));
}

void triangle_wave() {
lcd.clear();

byte image06[8] = {B00010, B00101, B01000, B10000, B00000, B00000, B00000, B00000};
byte image07[8] = {B00000, B00000, B00000, B10000, B01000, B00101, B00010, B00000};
byte image08[8] = {B00000, B00000, B00000, B10000, B00000, B00000, B00000, B00000};

lcd.createChar(0, image06);
lcd.createChar(1, image07);
lcd.createChar(2, image08);

lcd.setCursor(5, 0);
lcd.write(byte(0));
lcd.setCursor(6, 0);
lcd.write(byte(1));
lcd.setCursor(7, 0);
lcd.write(byte(2));
}


byte saw_wave[] = {
  B00001,
  B00011,
  B00111,
  B01111,
  B11111,
  B00000,
  B00000,
  B00000
};

byte saw_wave_hollow[] = {
  B00000,
  B00000,
  B00000,
  B00001,
  B00011,
  B00101,
  B01001,
  B10001
};

byte saw_wave_2[] = {
  B00011,
  B00011,
  B00111,
  B00111,
  B01111,
  B01111,
  B11111,
  B11111
};

byte saw_wave_2_hollow[] = {
  B00011,
  B00011,
  B00101,
  B00101,
  B01001,
  B01001,
  B10001,
  B10001
};

byte sine_part_1[] = {
  B00000,
  B00011,
  B00100,
  B01000,
  B10000,
  B00000,
  B00000,
  B00000
};

byte sine_part_2[] = {
  B00000,
  B00000,
  B10000,
  B01000,
  B00100,
  B00011,
  B00000,
  B00000
};

byte ramp[] = {
  B00000,
  B00100,
  B01100,
  B10100,
  B00101,
  B00110,
  B00100,
  B00000
};

byte triangle_1[] = {
  B00000,
  B00000,
  B00100,
  B11010,
  B00001,
  B00000,
  B00000,
  B00000
};

byte triangle_2[] = {
  B00000,
  B00000,
  B00000,
  B00001,
  B00010,
  B10100,
  B01000,
  B00000
};

byte triangle_3[] = {
  B00000,
  B00000,
  B10000,
  B01100,
  B00000,
  B00000,
  B00000,
  B00000
};

// byte triangle_1[] = {
//   B00000,
//   B00001,
//   B00010,
//   B00100,
//   B01000,
//   B10000,
//   B00000,
//   B00000
// };

// byte triangle_2[] = {
//   B00000,
//   B10000,
//   B01000,
//   B00100,
//   B00010,
//   B00001,
//   B00000,
//   B00000
// };

byte pulse_1[] = {
  B00000,
  B10000,
  B10000,
  B10000,
  B10000,
  B10000,
  B11111,
  B00000
};

byte pulse_2[] = {
  B00000,
  B11111,
  B10000,
  B10000,
  B10000,
  B10000,
  B10000,
  B00000
};

byte quarter_line_flat[] = {
  B00000,
  B00000,
  B00000,
  B00000,
  B00000,
  B00000,
  B11111,
  B00000
};

byte quarter_line_up[] = {
  B00000,
  B00011,
  B00010,
  B00010,
  B00010,
  B00010,
  B11110,
  B00000
};

byte quarter_line_down[] = {
  B00000,
  B11100,
  B00100,
  B00100,
  B00100,
  B00100,
  B00100,
  B00000
};

byte half_pulse[] = {
  B00000,
  B11111,
  B10001,
  B10001,
  B10001,
  B10001,
  B10001,
  B00000
};

byte tri_saw_1[] = {
  B00000,
  B11000,
  B10100,
  B10010,
  B10001,
  B10001,
  B10001,
  B00000
};

byte tri_saw_2[] = {
  B00000,
  B00010,
  B00010,
  B00010,
  B00100,
  B01000,
  B10000,
  B00000
};

void setup(){
  // initialize LCD
  lcd.init();
  // turn on LCD backlight                      
  lcd.backlight();
  pinMode(buttonPin, INPUT_PULLUP);
  // lcd.createChar(4, sine_part_1);
  // lcd.createChar(5, sine_part_2);
  
  //lcd.createChar(1, saw2);
}

int total = 7;
String waves[] = {"saw", "sine", "ramp", "triangle", "pulse", "quarter_pulse", "half_pulse", "tri-saw"};

void loop(){
  // set cursor to first column, first row
 
  buttonState = digitalRead(buttonPin);

  if (buttonState != lastButtonState) {
    if (buttonState == LOW) { // Button pressed
      currentCharIndex = (currentCharIndex + 1);
      if(currentCharIndex > total){
        currentCharIndex = 0;
      }
      String currentWave = waves[currentCharIndex];
      
      if(currentWave == "saw"){
        lcd.createChar(0, saw_wave_2_hollow);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(0);
        lcd.write(0);
        lcd.write(0);
      }
      if(currentWave == "tri-saw"){
        lcd.createChar(0, tri_saw_1);
        lcd.createChar(1, tri_saw_2);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(1);
      }
      if(currentWave == "half_pulse"){
        lcd.createChar(0, half_pulse);
        lcd.createChar(1, pulse_1);
        lcd.clear();
        lcd.home();
        lcd.write(1);
        lcd.write(0);
      }
      if(currentWave == "sine"){
      lcd.createChar(0, sine_part_1);
      lcd.createChar(1, sine_part_2);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(1);
        lcd.write(0);
        lcd.write(1);
      }

      if(currentWave == "ramp"){
      lcd.createChar(0, ramp);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(0);
        lcd.write(0);
      }

      if(currentWave == "triangle"){
      lcd.createChar(0, triangle_1);
      lcd.createChar(1, triangle_2);
      lcd.createChar(2, triangle_3);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(1);
        lcd.write(2);
      }

      if(currentWave == "pulse"){
      lcd.createChar(0, pulse_1);
      lcd.createChar(1, pulse_2);
        lcd.clear();
        lcd.home();
        lcd.write(0);
        lcd.write(1);
        lcd.write(0);
        lcd.write(1);
      }

      if(currentWave == "quarter_pulse"){
      lcd.createChar(0, quarter_line_down);
      lcd.createChar(2, quarter_line_flat);
      lcd.createChar(3, quarter_line_up);
      lcd.createChar(4, pulse_1);

        lcd.clear();
        lcd.home();
        lcd.write(4);
        lcd.write(2);
        lcd.write(3);
        lcd.write(0);
      }
      
    }
    delay(50); // Debounce delay
  }

  lastButtonState = buttonState;
}

---
Created: 2023-02-27T19:49
tags:
  - Arduino
---
```C++
/*
   注意事項:
   設計流程 : 好的timer -> 設定 pinMode -> Mode0~n -> C\#通訊
   timer1 計算 : 65535 - 16000000 / 256 / 500Hz(0.002)
   藍芽 AT command : {AT+NAME? AT+NAME=NAME AT+UART?s
                     AT+UART=2400,0,0 AT+PSWD=PWD AT+ORGL}
*/
\#include <Arduino.h>
\#include <avr/io.h>
\#include <util/delay.h>
\#include<avr/interrupt.h>
\#include <avr/wdt.h>
\#include <EEPROM.h>
\#include <NewPing.h>

int RunEveryTime = 16000000 / 256 / (1 / 0.025);

bool S1, S2, last_S1, last_S2;

byte Mode = 0;
String reading = "";

void setup() {
  Serial.begin(2400);
  Serial.setTimeout(15);
  SetupTimer1();
}
//===================================== Timer1  = ~2ms
void SetupTimer1() {
  cli(); //stop interrupts

  TCCR1A = 0;
  TCCR1B = 0;

  TCCR1B |= (1 << CS12);
  TIMSK1 |= (1 << OCIE1A);

  TCNT1 = 65535 - 125;
  TIMSK1 |= (1 << TOIE1);

  sei(); //enable
}
//===================================== 掃描按鈕
void ScanButton() {
  last_S1 = S1;
  last_S2 = S2;
  if (digitalRead(A0) == 0)
    S2 = 1;
  else
    S2 = 0;
  if (digitalRead(A1) == 0)
    S1 = 1;
  else
    S1 = 0;
}
//===================================== 主程式
void loop() {
}
//===================================== 每2ms 跑一次
byte ledindex = 0;
ISR(TIMER1_OVF_vect) {
  static int TimeRotateCnt = 0 ;
  TCNT1 = 65535 - RunEveryTime;
  wdt_reset();
  ScanButton();
  if (++TimeRotateCnt == 2) TimeRotateCnt = 0;
  switch (TimeRotateCnt) {
    case 1: case 0:
      if (Mode == 0) Mode0();
      if (Mode == 1) Mode1();
      if (Mode == 2) Mode2();
      break;
  }
}
```

- C# Formwork
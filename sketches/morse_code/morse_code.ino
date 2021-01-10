#define OFF_WAIT 100
#define LONG_WAIT 1000
#define SHORT_WAIT 250
#define LOOP_WAIT 1000

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
    delay(LOOP_WAIT);
    s(); o(); s();
    delay(LOOP_WAIT);
}

void blink(int off_wait, int on_wait) {
  delay(off_wait);
  digitalWrite(LED_BUILTIN, HIGH);
  delay(on_wait);                 
  digitalWrite(LED_BUILTIN, LOW);
  delay(off_wait);              
}

void long_blink() {
    blink(OFF_WAIT, LONG_WAIT);
}

void short_blink() {
    blink(OFF_WAIT, SHORT_WAIT);
}

void a() {
    short_blink();
    long_blink();
}

void b() {
    long_blink();
    short_blink();
    short_blink();
    short_blink();
}

void c() {
    long_blink();
    short_blink();
    long_blink();
    short_blink();
}

void d() {
    long_blink();
    short_blink();
    short_blink();
}

void e() {
    short_blink();
}

void f() {
    short_blink();
    short_blink();
    long_blink();
    short_blink();
}

void g() {
    long_blink();
    long_blink();
    short_blink();
}

void h() {
    short_blink();
    short_blink();
    short_blink();
    short_blink();
}

void i() {
    short_blink();
    short_blink();
}

void j() {
    short_blink();
    long_blink();
    long_blink();
    long_blink();
}

void k() {
    long_blink();
    short_blink();
    long_blink();
}

void l() {
    short_blink();
    long_blink();
    short_blink();
    short_blink();
}

void m() {
    long_blink();
    long_blink();
}

void n() {
    long_blink();
    short_blink();
}

void o() {
    long_blink();
    long_blink();
    long_blink();
} 

void p() {
    short_blink();
    long_blink();
    long_blink();
    short_blink();
}

void q() {
    long_blink();
    long_blink();
    short_blink();
    long_blink();
}

void r() {
    short_blink();
    long_blink();
    short_blink();
}

void s() {
    short_blink();
    short_blink();
    short_blink();
}

void t() {
    long_blink();
}

void u() {
    short_blink();
    short_blink();
    long_blink();
}

void v() {
    short_blink();
    short_blink();
    short_blink();
    long_blink();
}

void w() {
    short_blink();
    long_blink();
    long_blink();
}

void y() {
    long_blink();
    short_blink();
    long_blink();
    long_blink();
}

void z() {
    long_blink();
    long_blink();
    short_blink();
    short_blink();
}
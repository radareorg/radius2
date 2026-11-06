// zig cc -target mips-linux-musl -O1 -nostdlib -static -fno-pic -fno-inline -o delay delay.c
// mips branches, calls and returns all have delay slots
int sum(int n) {
    int s = 0;
    for (int i = 0; i < n; i++) {
        s += i * 3;
    }
    return s;
}

void win(void) {}
void lose(void) {}

int check(unsigned x) {
    if (sum(x & 0xf) == 135) {
        win();
        return 1;
    }
    lose();
    return 0;
}

void __start(void) {
    check(0);
    for (;;);
}

int foo(int x) {
    switch (x) {
        case 0:
            return 100;
        case 1:
            return 200;
        default:
            return x * 10;
    }
}

int main(void) {
    int total = 0;
    for (int i = 0; i < 5; i = i + 1) {
        total += foo(i);
    }
    return total;
}
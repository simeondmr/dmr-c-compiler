int main(void) {
    int a = 0;
    goto test;
    a = 10;
test:
    a = 11;
    return a;
}
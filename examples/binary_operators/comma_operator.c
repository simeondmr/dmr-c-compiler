int main(void) {
    int a = 0;
    int b = (a++, -4 * 2, ++a, 29 + 1);
    return b;
}
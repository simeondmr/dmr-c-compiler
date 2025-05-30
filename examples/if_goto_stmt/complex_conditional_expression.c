int main(void) {
    int a = 0;
    int b = 10;
    int c = a + (b & 2);
    int result = (c == 2) ? ++a, a + 4 : a + 2;
    return result;
}
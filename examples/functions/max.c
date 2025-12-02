int max(int a, int b) {
    if (a > b)
        return a;
    return b;
}

int main(void) {
    int val0 = 5;
    int val1 = 12;
    int val1 = 3;
    int val2 = 29;
    int val3 = 2;
    int m = val0;

    m = max(m, val1);
    m = max(m, val2);
    m = max(m, val3);
    m = max(m, val4);

    return m; // 29
}
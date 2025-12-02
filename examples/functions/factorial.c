int factorial(int n) {
    if (n == 1)
        return 1;
    return n * factorial(n - 1);
}

int main(void) {
    return factorial(4); // 4 * 3 * 2 * 1 = 24
}
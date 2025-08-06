int main(void) {
    int i = 0;
    do {
        ++i;
        if (i == 3) {
            continue;
        }
        i += 10;
    } while (i < 100);
    return i;
}
int main(void) {
    int a = 11;
    switch(a) {
        case 10:
            a = 20;
            break;
        case 11:
            a = 29;
            break;
        default:
            a = 0;
    }
    return a;
}
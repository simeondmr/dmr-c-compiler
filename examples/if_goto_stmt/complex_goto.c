int main(void) {
    int a = 2;
    int b = 2;
    if (a << 1 == b)
        goto end;
    goto error;
error:
    return 1;
end:
    return 0;
}
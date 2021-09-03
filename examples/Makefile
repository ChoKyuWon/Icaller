CC := clang
CFI_FLAGS := -fsanitize=cfi-icall -flto
CFLAGS := -g

all:
	$(CC) $(CFLAGS) $(CFI_FLAGS) test.c -o test.o
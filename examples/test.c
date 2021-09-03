#include <stdio.h>

int foo1(int* arg0){
    printf("This is foo1\n");
    return 0;
}

int foo(int* arg0){
    printf("This is foo\n");
    return -1;
}

int baz(int arg0, double arg1){
    printf("This is baz\n");
    return 0;
}

void bar0(int arg0, double arg1){
    return;
}

void bar1(int arg0, double arg1){
    printf("This is bar\n");
    return;
}

void bar(int arg0, double arg1){
    printf("This is bar\n");
    return;
}

char f0(){
    return 'a';
}

char f1(){
    return 'a';
}

char f2(){
    return 'a';
}

char f3(){
    return 'a';
}

char (*g_func)() = f0;

int main(){
    int (*func1)(int*) = foo;
    void (*func2)(int, double) = bar;
    int (*global_baz)(int,double) = baz;
    g_func = f3;
    f2();
    f3();

    printf("foo1 address:%p, foo address: %p\n", (void*)foo1, (void*)foo);
    
    scanf("%p", &func1);
    global_baz(0,0);
    func1(0);
    func2(0, 0.0);
    g_func();
}

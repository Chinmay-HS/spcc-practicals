%{
#define NUM 1
#define KEY 2
#define ID 3
%}

%%
"int"|"float"|"char"    { return KEY; }
[a-zA-Z_][a-zA-Z0-9_]* { return ID; }
[0-9]+(\.[0-9]+)?       { return NUM; }
[ \t\n]                 { /* ignore whitespace */ }
%%

#include <stdio.h>
#include <string.h>

const char* keywords[] = {"int", "float", "char"};

int is_keyword(const char* str) {
    int n = sizeof(keywords) / sizeof(keywords[0]);
    for (int i = 0; i < n; i++) {
        if (strcmp(str, keywords[i]) == 0) {
            return 1;
        }
    }
    return 0;
}

int main(int argc, char *argv[]) {
    int val;
    while ((val = yylex())) {
        switch (val) {
            case NUM: printf("Number: %s\n", yytext); break;
            case KEY: printf("Keyword: %s\n", yytext); break;
            case ID: printf("Identifier: %s\n", yytext); break;
            default: break;
        }
    }
    return 0;
}

int yywrap() {
    return 1;
}

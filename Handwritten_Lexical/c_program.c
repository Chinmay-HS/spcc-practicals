#include <stdio.h>
#include <float.h>

int main() {
    char op;
    double a, b, res;
    int valid = 1;
    printf("Enter operator (+ - * /): ");
    scanf("%c", &op);
    printf("Enter two numbers: ");
    scanf("%lf %lf", &a, &b);
    switch(op) {
        case '+': res = a + b; break;
        case '-': res = a - b; break;
        case '*': res = a * b; break;
        case '/':
            if(b == 0) {
                printf("Division by zero error\n");
                return 1;
            }
            res = a / b;
            break;
        default:
            printf("Invalid operator\n");
            valid = 0;
    }
    if(valid)
        printf("Result = %.2lf\n", res);
    return 0;
}

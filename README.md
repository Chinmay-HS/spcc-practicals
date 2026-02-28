# Compiler Design Laboratory Experiments

This documentation provides a technical guide for executing the compiler design experiments. The repository covers left recursive grammar detection, a handwritten lexical analyzer, and the use of the Lex tool.

---

## Table of Contents

1. [Experiment 1: Left Recursive Grammar](https://www.google.com/search?q=%23experiment-1-left-recursive-grammar)
2. [Experiment 2: Handwritten Lexical Analyzer](https://www.google.com/search?q=%23experiment-2-handwritten-lexical-analyzer)
3. [Experiment 3: Lex Tool](https://www.google.com/search?q=%23experiment-3-lex-tool)

---

## Experiment : Left Recursive Grammar

This experiment involves a Rust program designed to determine if a context-free grammar (CFG) is left recursive.

### Setup and Execution

1. Create a file named `left_recursion.rs` and paste the implementation code.
2. Open the terminal and navigate to the directory containing the file.
3. Compile the code:
```bash
rustc left_recursion.rs

```


4. Execute the binary:
```bash
./left_recursion

```



### Sample Input

```text
Enter number of productions:
2
Enter productions in format: A -> Aa | b
E -> E+T | T
T -> T*F | F

```

### Expected Output

```text
E is Left Recursive
T is Left Recursive

```

---

## Experiment : Handwritten Lexical Analyzer

This experiment demonstrates a custom lexical analyzer written in Rust that parses C source code and identifies tokens such as keywords, identifiers, and delimiters.

### Setup and Execution

Ensure the Rust environment is set up and the source file `Calculator.c` is located in the `src` directory. Use the following command to pipe the C code into the lexical analyzer:

```bash
type src\Calculator.c | cargo run

```

*(Note: On Linux, use `cat src/Calculator.c | cargo run`)*

### Sample Input (Content of Calculator.c)

```c
int main() {
    char op;
    double a, b, res;
    int valid = 1;
}

```

### Expected Output

```text
<KEYWORD int>
<IDENTIFIER main>
<DELIMITER (>
<DELIMITER )>
<DELIMITER {>
<KEYWORD char>
<IDENTIFIER op>
<DELIMITER ;>
<KEYWORD double>
...

```

---

## Experiment : Lex Tool

This experiment utilizes the Lex tool to generate a lexical analyzer automatically from a specification file (`.l`).

### Setup and Execution

1. Create a Lex specification file (e.g., `rulefile.l`).
2. Generate the C source code for the lexical analyzer using the lex tool:
```bash
lex rulefile.l

```


*(Alternatively, use `flex rulefile.l`)*
3. Compile the generated `lex.yy.c` file using a C compiler:
```bash
gcc lex.yy.c -o lex

```


4. Run the executable:
```bash
./lex

```



### Sample Input

```text
hello
23
else

```

### Expected Output

```text
ID
NUM
KEYWORD : else

```

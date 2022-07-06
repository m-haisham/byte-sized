# Paroxy

Brainfuck like, slightly more user friendly compiled language written in rust.

## Features

- Write strings in code and populate cells accordingly.
- Print current cell or a determinate number of following cells.
- Input and populate multiple cells at a time.
- Increment/Decrement cell using a defined value.
- Set a certain number to the current cell.
- Move pointer with a defined value.
- Move to a cell of your own choosing with their index.
- Fully compatible with brainfuck.

## Grammar

### Write string to tape

```text
STRING '$'? '^'?
```

**STRING (required):** string literal terminated with either `"` or `'`.

**$ (optional):** print the preceding string literal to the terminal.

**^ (optional):** move the pointer to just after the string literal.

#### Write string example

The below code will populate the current cell and the following. The following `$` would print the string to the terminal. And the `^` would move the pointer just after the
string populated cells.

```text
"Hello World!"$^
```

### Print

```text
'.' NUMBER?
```

**. (required):** dot initiating print expression.

**NUMBER (optional)**: number literal denoting number of following cells to print. Default is 1.

#### Print example

The below code will print the range from 0 (current cell) to 4 (included).

Equivalent to `0..5`.

```text
.5
```

### Input

```text
',' ('*' '^')?
```

**, (required):** a comma initiating input expression.

**\* (optional):** denote expression as multi character input.

**^ (optional):** move pointer to just after the input.

#### Example

The below code will write the full input text into the adjacent cells and move the pointer just after.

```text
,*^
```

### Cell increment/decrement

Current cell value can be incremented by `+` and decremented by `-`.

```text
('+' | '-') NUMBER?
```

**NUMBER (optional):** increment/decrement by this value. Default is 1.

#### Increment example

```text
+10-5
```

### Set the cell number

```text
'#' NUMBER
```

**NUMBER (required):** a number literal between 0 (included) and 255 (included).

### Move pointer

Pointer movement is controlled by `<` for left or `>` for right followed by a number.

```text
('<' | '>') NUMBER?
```

**NUMBER (optional):** number of cells to move.

#### Move pointer example

```text
>10<5
```

### Move to specific cell

```text
'@' NUMBER
```

**NUMBER (required):** index of the destination cell.

#### Specific cell example

```text
@4
```

### Loop

A loop in paroxy starts with `[` and ends with `]`. All the expressions encapsulated are repeated while the current cell value is not 0.

```text
'[' expression* ']'
```

**expression (zero-or-more):** the expressions to be repeated.

> Note: All paroxy actions are expressions.

#### Loop example

```text
+[>+<-]>.
```

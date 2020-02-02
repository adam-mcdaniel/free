# free

A terrible programming language that targets an even worse programming language.

I decided to name this programming language free because there are absolutely no memory constraints whatsoever. It's basically impossible for a free program to segfault or anything like that: you can arbitrarily assign to any memory location. This isn't by design. It's just a byproduct of the target programming language: SMPL.

# SMPL

[SMPL](https://github.com/adam-mcdaniel/smpl), pronounced "simple", is a programming language almost identical to brainfuck. It's ridiculously easy to implement; SMPL is just a superset of brainfuck with 3 additional operators. These operators, `&`, `*`, and `?`, give SMPL the ability to manage memory dynamically rather than statically, which is impossible in brainfuck. Consider the following problem.

In brainfuck, it's incredibly easy to represent an array of numbers. They can just be stored on the tape like so.
```c
char array[6]
          |
          v
[0, 0, 0, 1, 2, 3, 4, 5, 0, 0, 0, ...]
```

But what happens when the array needs to grow? Or if you want to represent a pointer to an element of the array?

There is no real elegant solution to representing pointers in brainfuck.

However, enabled by the powers of SMPL we can represent pointer operations rather elegantly.

# Compilation Process

After the input code has been parsed and the AST has been generated, the compilation can begin.

Honestly, I'm not sure I still understand the complete scope of how the entire compiler works. A lot of it seemingly works by magic. Nevertheless, I'll still try to give a meaningful explanation. **_Do not trust this software because I honestly have no idea why it works_**.

1. Initialize the stack and the heap
    - When compilation begins, the compiler sets asside memory that it **KNOWS** will be necessary. This memory consists of the _return_ register, and six other registers for math. Although it may appear that I chose six registers because most CPUs have six registers, I actually chose six because that is the minimum number of temporary cells required for any atomic math algorithm in brainfuck listed on esolangs.org.

    - After the stack memory is allocated, the total memory allocated for the combined stack and heap is calculated.

2. Inline function calls
    - Essentially, all functions' bodies are copied and pasted into where they are called.
    
    - Even though functions are inlined, they still have stackframes which allocate and deallocate function memory before and after calls.

3. Statically compute stack allocations
    - This is probably the **_worst_** part of the compilation process. It's **entirely** unnecessary and I probably should've just made a predetermined stack limit such as `8192` bytes or something.

    - When a scope is instantiated, a new scope is pushed onto the current scope hierarchy. When variables are defined in the scope, they are allocated in the environment's stackframe. When the scope ends, every variable in the scope is deallocated. Notice how I never mentioned free's stack pointer in the list of registers? That's because there **_is no stack pointer_**. All stack memory is accounted for **during compile time**. That means that for every scope, the storage capacity for the stack grows **exactly** as much as is necessary. The compiler can compute exactly how much memory will be allocated on the stack at any arbitrary point of time in the runtime.

    - I hate myself for implementing the stack this way. Never do this. Just implement `push` and `pop` and be done with it. **_Don't be like me, you'll regret your stupidity_**.

4. Convert atomic operations into SMPL/brainfuck
    - I used esolangs.org page on brainfuck algorithms for this and it was incredibly handy. Building an intermediate representation as a superset of brainfuck is actually a really great way to have a bunch of pre-built algorithms at your disposal. I found this part extremely simple and easy to implement. The hardest portion was implementing pointers **correctly**. There were very many times where my test programs would have inexplicable errors, and it was _always_ the algorithms responsible for representing pointers in SMPL. In fact, there are probably a reasonable number of memory bugs **still** in the compiler.

    - The best way to avoid errors is honestly just to enable brainfuck compatibility mode. Not only will this allow you to run your code with industrial strength brainfuck compilers and interpreters, it will free you from banging your head against a wall over weird memory behavior.

5. Optimize SMPL/brainfuck
    - Because SMPL and brainfuck are so minimal, optimizations are extremely effective. Little things like zeroing deallocated memory are _technically_ unnecessary, and can be optimized away if needed.

    - This is the last step in the compilation process before SMPL is optionally converted to C.

After all these steps, your compiled `free` program is ready to execute.

# Syntax and Flags

The syntax of free is heavily inspired by rust. This is because of the objective fact that rust is the best programming language ever made.

Every free program contains a `start` function (like the `main` function).

```rust
fn start() {
    // This is a variable declaration
    def str = "Hello world!";
    println(str);
}
```

I **_REALLY_** wanted to use the `let` keyword because it's so pretty, but no variable in free is constant. I used `def` because it's less misleading, and because `var` is uglier in my opinion.

free also has two different control flow structures: the while loop, and the if-else statement.

The if-else statement functions the same way as an if statement in any other language. There are no else-if expressions, though.

```rust
fn start() {
    if test() {
        // this code will run if test() is non-zero
        println("True!");
    } else {
        // this code will run if test() is zero
        println("False!");
    }
}

fn test() {
    return 1;
}
```

While loops are much different, however. Because of the way free's memory is managed, it isn't possible to use a function call as the condition to a while loop. Variables **must** be used to store the condition of a while loop because their position in the memory tape is always constant within their scope.

```rust
fn start() {
    def running = 1;
    while running {
        println("running!");
    }
}
```

For the same reason that while loops require variables for their conditions, only variables can be referenced. Any value can be dereferenced, though.

```rust
fn start() {
    def a = 5;
    // print `a` as a digit by adding ascii code for 0
    println(add(a, 48));
    inc(&a);
    // print `a` as a digit by adding ascii code for 0
    println(add(a, 48));
}

// No type is needed for `ptr`. All variables are typeless because type checking is hard.
fn inc(ptr) {
    *ptr = add(*ptr, 1);
}
```

Using the `alloc` function, now we can use dynamic memory allocation!

```rust
fn start() {
    // Allocate 16 bytes of memory and store the pointer to that block in `str`
    def str = alloc(16);

    if 1 {
        *str = "True!\n\0";
    } else {
        *str = "False!\n\0";
    }

    cprint(str);
}

fn cprint(str) {
    def counter = 0;
    def running = *add(str, counter);
    while running {
        print(running);
        counter = add(counter, 1);
        running = *add(str, counter);
    }
}
```

We also need to be able to free memory that we allocate.


```rust
fn start() {
    // Allocate 128 bytes of memory and store the pointer to that block in `str`
    def size = 128;
    def str = alloc(size);
    free(str, size);
}

// free_byte only frees a single cell, so free must be implemented manually
fn free(ptr, size) {
    while size {
        size = sub(size, 1);
        // free_byte is built in
        free_byte(add(ptr, size));
    }

    // Store 0 in the return register
    return 0;
}
```

# Sample Output

Now to show you some god awful output code.

Here is hello world in free.
```rust
// This flag enables brainfuck compatibility mode.
// This disables pointer operations and any number literal greater than 255
#[enable(brainfuck)]

fn start() {
    println("Hello, world!");
}
```

This gets compiled to the following.

**_Forgive me for what I've created._**

```brainfuck
[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++>+<>>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<[-]>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<+>>>>>>>-]<<<<<<<[>>>>>>>+<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>-]<<<<<<<<[>>>>>>>>+<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>-]<<<<<<<<<[>>>>>>>>>+<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>-<>]<<<<<<<<<<[>>>>>>>>>>+<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>-]<<<<<<<<<<<[>>>>>>>>>>>+<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>-<>]<<<<<<<<<<<<[>>>>>>>>>>>>+<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>-]<<<<<<<<<<<<<[>>>>>>>>>>>>>+<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>.>.>.>.>.>.>.>.>.>.>.>.>.>.<<>>>[-]++++++++++.<<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<>>>[-][-]<<<<<<<<<<<<<<<<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<
```

**Yikes** that's bad. It does, however, fairly simply show how the compilation process works. Let me break it down.

This is the first bit of output code from the compiler, and it's the simplest part to grasp. Here, the compiler is allocating and zeroing the register cells in addition to the cells that will be used for storing the string literal on the stack.

```brainfuck
[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<
```

Then, the compiler actually assigns the data from the string literal to the memory location on the stack.


```brainfuck
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++>+<
```

Then, the compiler copies the string over to a new memory location for the `println` function to manipulate.


```brainfuck
>>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<[-]>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<+>>>>>>>-]<<<<<<<[>>>>>>>+<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>-]<<<<<<<<[>>>>>>>>+<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>-]<<<<<<<<<[>>>>>>>>>+<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>-<>]<<<<<<<<<<[>>>>>>>>>>+<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>-]<<<<<<<<<<<[>>>>>>>>>>>+<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>-<>]<<<<<<<<<<<<[>>>>>>>>>>>>+<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>-]<<<<<<<<<<<<<[>>>>>>>>>>>>>+<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<[>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>-]<<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<-]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[-]<<<<<<<<<<<<<<<>[<>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>>>>>>-<>]<<<<<<<<<<<<<<<<<<<<[>>>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<<-]
```

And lastly, the program prints out the string and cleans up the stack.

```brainfuck
[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<>>>[-][-]<<<<<<<<<<<<<<<<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<<<<<<<<<<<<<<<[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<
```

Although most of this output code is understandable, there are still a few sections that are confusing to me.

Like I'm not completely sure why `>+<` is there when the program is storing the `Hello world!` string, but whatever. It works, and I've drained my capacity to care why.
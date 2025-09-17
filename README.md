## 😃 Intro

Ion is a functional, statically typed - Interpreted - programming language written from the ground up in Rust;  
Offering Zero-Cost Abstractions along with Numeric Type Minimization with ease of workflow in an attempt to give you all the perks of Rust's type system but without any extra effort on YOUR part - It aims to simulate Rust's Borrow Checker Ideology and Safe Handling with its own variation for the same, along with a powerful and well adjusted | Recursive Descent LL (1) Parser |. Fast, Practical, Easy to read! ION.
Too lazy to try it out yourself? Feel free to run the examples! Tinkering with them however so want;

<h2>💭 Our Ideaology</h2>
<p>The Major Difference b/w how Rust handles memory and Ion is that unlike Rust, Ion doesn't dump all the responsibility of handling memory on the programmar; For example each variable you define will have its value owned by a central mem. management system and to access the same, you will only be able to "borrow" it [Note: Upon mutation however, a shallow copy of the same will be created and used until it is used to re-assign the target variable; Otherwise, the Referance would be in Sync with the original value in the C.M.M] ; Any time you wish to mutate it a check will run through making sure no other point in the runtime is trying to access the same; think of it as a Mutex but without the overhead that comes w/ it! </p>
<hr>
<hr>

##### Currently Supported Data Types
- [x] Numeric ~ u8 | u16 | u32 | u64 || i8 | i16 | i32 | i64 | f32 | f64 [Automatically Compressed During Interpretation]
- [x] String
- [x] Bool
- [x] Complex ~ [Anonymous], Object, Array
- [x] Structs ~ Object, Array

##### Currently Supported
- [x] Member Expressions - Dot Notation (x.y)
- [x] Comments - Single Line (//) | Multiline (/* */)
- [x] Array Indexing (arr[i])
- [x] String Concatenation
---
---

## 🚀 Setup [WSL] 

[In case of Bash or Fsh or Other, base commands should be the same, feel free to format them if needed for they are already simple...]

For the setup, just run:

```bash
git clone https://github.com/hurshbajaj/Ion && cd Ion && make install && make build
```

## ▶️ Try the First Example

To try out the first example, feel free to copy paste the contents of (t1.io) into main.io and:

```bash
make run
```
<br>

## ✨ Author's Note

This project is written entirely by me, including this message —  

with AI support being limited to light debugging (outside the project whilst testing nodes separately) (NOT within the project itself.), conceptual understanding for me whilst writing this and this message's MD formatting inspo (and inspo alone.);
(Yes... the emojis used are intentional)  

<hr>

If you wish to see the debug logs / how Ion breaks down and safely parses wtv you throw at it, go into src/runtime/mod.rs and set the PRINT_ variable to true...
**Thank you!**



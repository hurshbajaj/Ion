## :D Intro

Ion is a functional, statically typed - Interpreted - programming language written from the ground up in Rust;  
Offering Zero-Cost Abstractions along with Numeric Type Minimization with ease of workflow in an attempt to give you all the perks of Rust's type system but without any extra effort on YOUR part - It aims to simulate Rust's Borrow Checker Ideology and Safe Handling with its own variation for the same, along with a powerful and well adjusted | Recursive Descent LL (1) Parser |. Fast, Practical, Easy to read! ION.
Too lazy to try it out yourself? Feel free to run the examples! Tinkering with them however so want;

---

---

The Major Difference b/w how Rust handles memory and Ion is that unlike Rust, Ion doesn't dump all the responsibility of handling memory on the programmar; For example each variable you define will have its value owned by a central mem. management system and to access the same, you will only be able to "borrow" it; Any time you wish to mutate it a check will run through making sure no other point in the runtime is trying to access the same; think of it as a Mutex but without the overhead that comes w/ it!
<br>  
As a side note, however, certain data types such as numeric will be treated as < Owned > values for ease of workflow and the programmar; even when taken out from a referanced variable, and the value will only be mutated once u explicitely set the variable to the same (It's a lot to take in! Refer to the tests if you can.)  

---
---
##### Currently Supported Data Types 
- Numeric ~ u8 | u16 | u32 | u64 || i8 | i16 | i32 | i64 | f32 | f64 [Automatically Compressed During Interpretation]
- Bool
- Complex ~ Object

###### [For further Data Types read Author Note]

---
---

## 🚀 Setup [WSL] 

[In case of Bash or Fsh or Other, base commands should be the same, feel free to format them if needed for they are already simple...]

For the setup, just run:

```bash
git clone https://github.com/hurshbajaj/Ion && cd Ion && make install && make build
```

## ▶️ Try the First Example

To try out the first example, feel free to run:

```bash
make run file="examples/t1.io"
```

Or if you want to let your own creativity run wild, feel free to

```bash
make run file="examples/main.io"
```
[Editing it however so you want]

## 💡 REPL Mode

Moreover, we even have a REPL — if you're simply looking for some light play with my project 🤗🤗🤗

```bash
make run 
```

---

## ✨ Author Note

This project is written entirely by me, including this message —  

with AI support being limited to light debugging, conceptual understanding for me whilst writing this and this message's MD formatting inspo;
(Yes... the emojis used are intentional)

As a side note, this project is simply a scaffold, although with an extensive design...  
> For example for parsing equality operators you can easily take bin op as referance... equality ops would infact be even easier to implement due to the lack of need of recursively handling the same! Complex types such as arrays can be implemented w the help of existing complex - kinds as referance (Object Struct / Object Literals), whereas for Strings you would only majorly have to change the Lexer (Don't worry - The way the project has been structured makes it VERY simple) without the need of a referance whatsoever as the lexer already takes care of all the logic bumps! Implementing Strings should be as easy as mentioning the type - letting the interpreter know that it exists (= and you should be good to go! 

The base that has been implemented however is as good as it gets at this level and all realistic edge cases have been taken care of so no need for you to worry as far as the scaffold is concerned :D Have fun!  
<br>
If you don't wish to see the debug logs / how Ion breaks down and safely parses wtv you throw at it, go into src/runtime/mod.rs and set the PRINT_ variable to true...
**Thank you!**


This is a continuation of the Udemy course Rust Programming Language for Beginners. This repository will include taking user input, conditional statements, loops, basic functions, tuples, and arrays in Rust. Details of these subjects will be broken out individually, with functional examples in the basics.rs file in this repository and the explanations here in the README.

OBTAINING USER INPUT

Rust is somewhat verbose in acquiring use input; it also can only acquire that input as a string data type. To save user input as a different data type, the input must be cast as the desired data type. As long as the input value is a valid sample of the desired data type, the input can be saved as that data type.

In order to acquire user input, Rust requires use of its standard library's io package. This can be harnesses by the line

`use std::io;`


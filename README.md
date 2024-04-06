# OS_Kernel
An x86_64 kernel written in Rust, designed for safety and performance. Exploring low-level system operations and concurrency, this project aims to deepen understanding of operating system fundamentals.



## Usage

Build the target: 

```terminal
cargo build --target thumbv7em-none-eabihf
```

Run based on your OS: 

```terminal
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"

```


## Learnings

This section is written about everything I have learned

### A free standing binary in Rust

A OS is simple a executable that runs on "bare metal". In this context, it means to have no support from other software.

To create the freestanding binary we need to remove access to the standard library. Note that the majority of functions will be removed and unusable. However, there are still some core functionality that is liked to the programming language. 

To ignore the standard library, we use the `no_std` macro that is built it. This will lead to some errors. Printing the console is not included. Neither is error handling. Therefore, we create a error handling function using the built in macro. We also tell the `Cargo.toml` file to abort if there is an error. 

Main is actually not the first function to be called. For all programming languages that has a runtime system, they need to start the runtime first. In the case of Rust, it needs to start a runtime created in C. The executable do not have access to the runtime, we need to create a new entry point. We add the `no_main` macro, to tell the compiler that are not using a normal entry point (`fn main`). At this point, one can completely remove the main function.

The next thing is to define the `_start` function. This function is called with the C Calling convention. Read more about [calling convention](https://en.wikipedia.org/wiki/Calling_convention), This function should never end - i.e it uses the `!` to signify never. Why? Well because this is the entry point the bootloader calls. For exiting the program, you should use the exit system call. This makes sense!

After defining the function, the next error is a linking error. Linker is a program that combines generated code with the executable. The error comes from the runtime is expected to be the C runtime, but it is not. To solve this, we just need to build for a bare metal target. 


## Specs

Here is my setup that was used for running and creating the kernel:
![image](https://github.com/indrehusdev/OS_Kernel/assets/66110094/0623c42b-7f0b-46a0-9de2-6500ebc77e21)



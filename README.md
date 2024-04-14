# OS_Kernel

![image](https://github.com/indrehusdev/OS_Kernel/assets/66110094/c10c44d0-6d75-4506-bcff-4b1bcfa8cbc3)


An x86_64 kernel written in Rust, designed for safety and performance. Exploring low-level system operations and concurrency, this project aims to deepen understanding of operating system fundamentals.



## Usage

Setup Rust Nightly with the following command (see `rust-toolchain` file):
```terminal
rustup show
```

Build the target: 

```terminal
cargo build --target x86_64.json
```

Run the build: 

```terminal
cargo run
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

The error happens because the underlying linker thinks that it is compiling for an underlying operating system. It is not. We can use the following commands to build to a triple target.


```terminal
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

When we add the `--target` argument, we cross compile for bare metal target on the system. 

### A minimal rust kernel

This repository creates a kernel with rust for a 64 bit x86 architecture. When you boot up the computer. It starts executing from firmware code that is stored in the ROM (read only memory). 

The next step is to execute the Power-on self-test (POST). This tests looks for RAM and pre-initializing the CPU. Then it starts to look for the operating system kernel. 

x86 has two firmware standards:
- **BIOS** - Basic Input/Output System
- **UEFI** - Unified Extensible Firmware Interface

For this project, the BIOS will be used. It was created in 1980 and is therefor more supported than most. The UEFI is modern, has more features, but are more complex to implement. 

When the computers starts, it is the BIOS that starts up and runs the self test, and initializes the hardware. Then it looks for a bootable disk. If found, it transfers control over to the bootloader. The bootloader takes up  512-bytes of executable code. It is stored in the beginning of the disk. If a bootloader is larger than 512B, then the bootloader is split into two stages. The bootloader has to determine the location of the kernel image on the disk. 

The CPU has to to transitions between what mode it is in:
- 16-bit real mode
- 32-bit protected mode
- 64-bit long mode

Real mode is a operating mode for all x86 CPUs. It gets the name from the fact that the addresses in real mode, is always corresponding to real locations in memory. It gives software unlimited access to all addressable memory. 

Protected mode is also a operating mode for all x86 CPUs. It is a mode that allows segmentation, virtual memory, paging and safe multitasking. When a software goes to protected mode, it also has to support real mode for backwards compatibility.  

Long mode is the last operating mode where the 64 bit operating system can access 64 bit instructions and registers.

Bootloader are written in assembly. This project will not create its own bootloader with assembly. However, I have written it down in my bucketlist of projects I want to create :-). For this project, a tool called `bootimage` is going to be used to automatic add the bootloader to kernel.

To make a bootloader that is compatible for every operating system, you can use the one created by the Free Software foundation. It is called `Multiboot` and it creates an interface between the bootloader and the operating system. To set it up, all we need is a Multiboot header at the beginning of the kernel. Note that there are some problems with both documentation, page sizing and what CPU mode is supported. So in this project does not use the Mulitboot, because of compatibility. 

In this project, we are going to switch to Rust nightly. This is already setup. All you need is to run the following command: 

![image](https://github.com/indrehusdev/OS_Kernel/assets/66110094/e5016a36-5341-4c01-91d4-a099e0db71c8)

The reason why we add it, it so that we can use experimental features. 

We know the cargo build tool supports different targets. We need to setup the target so that no underlying OS is being used. The configuration of setting this up can be found in the `x86_64.json`

The `llvm-target` is set to the correct architecture with no os. We also specify that we are going to use the Rust linker. We also set that in case of `panic`, the program should abort. We also disable red zone. The config does also add and remove certain features. The features for SIMD are disabled. It is used to significantly improve programs. However, it is can lead to large performance issues. 

Next step is to add the `Core` library to the target. This library contains features like `Result` and `Option`. This library is not setup for compiling to our costume target. The solution is by adding an experimental feature to the `Cargo.toml` file. It will recompile the library.

To build, you will ned to setup the config for cargo. I created a file in `.cargo/` called `config.toml`. This file includes configuring the std and setting the default target for this repository. 


Lets add text to the screen using VGS buffer. It is a special buffer that is mapped to the VGA hardware and contains a set of content. The VGA buffer starts at `0xb8000`. We create a variable for the text as a byte array: 

```rust
static HELLO: &[u8] = b"Hello World!";
```

Then we iterate over each character in the byte array and set the color of the corresponding character. We install the `bootimage` dependency. We can use it with the following command to setup a bootable image: `cargo bootimage`. Bootimage combines the bootloader and your kernel into a bootable disk image.

The following steps are taken during creating of the bootable image: 
- It creates a ELF: Executable and Likable format. It is a file format for executables. 
- It compiles the dependencies as a standalone executable.
- It links the bytes of the kernel ELF file to the bootloader.

Now, to start QEMU, a way to boot the bootable disk: 
https://www.qemu.org/download/

Boot it with the bootable QEMU:
```terminal
qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-os-kernel.bin
```

### VGA Text Mode 

The VGA text mode is a very simple way of printing information to the screen. The VGA text bugger is a two dimensional array with 25 rows and 80 columns for rendering the screen. Each array has the following information:

- ASCII Code point => 0-7 bits 
- Foreground color => 8-11 bits
- Background color => 12-14 bits
- Blink => 15 bit

The first byte is for representing the ascii character. The next byte is for representing the the foreground color. Then the next 3 bit for background color and then the last bit is for if the ascii character should blink or not. The VGA Buffer is available via the address `0xb800`. It is not in RAM, but directly accessible as write and read in the VGA hardware. '

We need to represent each color. We can do this by first creating the enum that represents the a color option: `pub enum Color{}`. Each value is stored as a `u8`. The article also specifies that we do not need a whole byte to represent each option. However, `u4` does not exist in rust. Then we create a structure that represent a color byte. The byte will include information about foreground and background color. The following code is the implementation for creating a new color byte: 

```rust
impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}
```

The byte is created by first adding th background color. Then **shifting it left by 4 bits**. Then doing the or operator with the foreground color. That will combine the two colors into a single byte, where the background color comes first and then the foreground color. Each color using 4 bits. 

Next two new structs - `ScreenChar` and `Bugger`. The ScreenChar is for a single character. It contains the ascii code and th color code. The buffer represents the text buffer. We set the size of the buffer, so that it has 25 rows and 80 columns. By using the macro `repr(C)`, we make the struct structure be in the order as a C structure. The order of the fields are important for the screen character.  

### Unit and Integration testing in no_std executables

Rust has a built-in test framework. This framework does not need to be setup. It is simply as easy as adding the macro for testing and then run `cargo test`. However this is standard library. Testing with `no_std` is more difficult, but this project has some test. This is implemented with the `custom_test_frameworks` feature.  


A cool feature within rust is the ability to conditional compilation. By using `#[cfg(..)]` we can tell the compiler to conditionally render code based on the flags provided to the compiler. In our case, we can specify that if the `test` flag is set we run the test code. 

## Specs

Here is my setup that was used for running and creating the kernel:
![image](https://github.com/indrehusdev/OS_Kernel/assets/66110094/0623c42b-7f0b-46a0-9de2-6500ebc77e21)

## Resources

This was not possible without the great resources from:
https://os.phil-opp.com/

I recommend checking out his blog series. 

Here is also a list of other topics that found me interesting during the implementation: 

Read-only memory for memory that is not changed by software: <br>
https://en.wikipedia.org/wiki/Read-only_memory

Power-on self-test. The first thing a computer does on startup: <br>
https://en.wikipedia.org/wiki/Power-on_self-test 

BIOS firmware to provide runtime for the OS: <br>
https://en.wikipedia.org/wiki/BIOS 

Real mode: unlimited access for the software to all addressable memory. Research also protected and long mode, if this was interesting. <br>
https://en.wikipedia.org/wiki/Real_mode

Red zone, what is it: <br>
https://os.phil-opp.com/red-zone/

SIMD: <br>
https://en.wikipedia.org/wiki/Single_instruction,_multiple_data

VGA Text Mode: <br>
https://en.wikipedia.org/wiki/VGA_text_mode

Executable and Linkable format: <br>
https://en.wikipedia.org/wiki/Executable_and_Linkable_Format 


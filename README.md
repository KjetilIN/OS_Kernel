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

In this project, we are going to switch to Rust nightly: 

![image](https://github.com/indrehusdev/OS_Kernel/assets/66110094/e5016a36-5341-4c01-91d4-a099e0db71c8)







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

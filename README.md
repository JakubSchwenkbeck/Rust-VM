# 16 Bit Virtual Machine in Rust

Welcome to the **16 Bit Virtual Machine in Rust** project! This project serves as a hands-on learning experience for Rust, focusing on building a virtual machine with custom features. The VM includes a MIPS-like assembler, custom registers, memory management, a file system, and a command-line interface (CLI) for executing code.

## Overview

The project is inspired by TomMarksTalksCodeLive and aims to explore Rust’s capabilities by developing a 16-bit virtual machine. The VM features its own assembler language and decoder, enabling low-level machine code access. The project leverages Rust’s robust backend features to create a fully functional virtual machine with a comprehensive OS-like environment.

## Features

- **16-bit Virtual Machine:** A virtual machine that simulates a 16-bit architecture.
- **MIPS-like Assembler:** A custom assembler for writing machine code.
- **Custom Registers:** Implemented CPU registers specific to the VM.
- **Memory Management:** Efficient memory allocation and management within the VM.
- **File System:** Basic file operations including file creation.
- **Command-Line Interface (CLI):** For user interaction and code execution.
- **Text Editor:** An integrated text editor for editing assembler code.
- **Assembler Code Parsing:** Parses and executes code written in a text-based assembler format.

## Project Details

### Components

1. **Assembler and Decoder:** Parses `.txt` files in assembler format and converts them into executable machine code.
2. **CPU Registers:** Custom registers designed for the VM’s CPU.
3. **Memory Management:** Handles allocation and management of memory resources.
4. **File System:** Supports file creation and basic file operations.
5. **Command-Line Interface:** Provides a CLI for running programs and interacting with the VM.
6. **Text Editor:** Allows users to create and edit assembler code directly within the VM environment.


# Rust CLI Launcher for Lunar Client
This is a Rust CLI launcher for the Lunar Client, a popular modpack for Minecraft that enhances gameplay and provides various features like FPS boost, mods, and more. This launcher allows you to quickly launch the Lunar Client from the command line.

## Prerequisites

Before using this Rust CLI launcher, make sure you have the following:
- Rust programming language installed on your system. You can install it from the official Rust website.
- Lunar Client installed on your system. You can download it from the official Lunar Client website.

## Usage

To use this Rust CLI launcher, follow these steps:
1. Clone this repository to your local machine using the following command:
```
git clone https://github.com/Moose1301/LunarCli.git
```
2. Navigate to the cloned directory using the following command:
```
cd LunarCli
```
3. Build the Rust executable using the following command:
```
cargo build --release
```
Or for testing:
```
cargo run -- --version <VERSION>
```
4. Run the executable using the following command:
```
.\target\release\LunarCli.exe --version <VERSION>
```
This will launch the Lunar Client on your system.

## Options
This Rust CLI launcher provides the following options:
- --version: Allows you to select the version you wan't to launch.

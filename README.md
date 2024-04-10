# Journal command line tool written in rust

## What is this?

I have recently been learning rust and wanted to make a little tool to learn how to use CLAP. I figured a simple journal program would be great as I have been wanting to start journaling, and I made it so you can use any editor you want, allowing me to use my Neovim config.

## Installation

Make sure you have this added to your path

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then you you should just be able to run the install script

```
./install.sh
```

## Usage

Once you have installed the program, you should just be able to run it using the **journal** command

```
journal
```

This will automatically create a **journal/** folder in your home directory and prompt you which editor you want to use. If you ever want to change the editor being used you can change it in the editor_config.txt file in the **journal/** directory. Hope you enjoy!

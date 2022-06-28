# Number to Word Convertor (cro)

CLI program that converts numbers to words in Croatian language.

## Note before use

It is recommended to run this program in a terminal which supports colors.

## Setup

First, clone the source code and enter it's directory.

Then build the '*.exe*':

```console
> cargo build --release
```

After the build process is done, go to folder:

```console
number-to-word-converter
│   Cargo.toml
│   README.md
|   ...
│
└───src
│   │   main.rs
│   
└───target
    │   release
    │   |   **number-to-word-converter.exe**  <--
    │   |   ...
    │   ...
```

Copy the highlighted '*.exe*' to some more accessible place and rename it if you want a simpler command name.

## How to Run

Open your terminal in the same directory where the '*.exe*' is and type:

```console
number-to-word-converter 435
```

Your output should look like this:

```console
Konvertirano:
        četiristo trideset pet
```

The command name which triggers this function can be whatever you name your '*.exe*' file (*\<name_of_.exe_file> \<any_number>*).

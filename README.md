# Number to Word Convertor (cro/hr)

CLI program that converts numbers to words in Croatian language.

## Note before use

It is recommended to run this program in a terminal which supports colors.

## How to install

You can easily download the executable from the [releases tab](https://github.com/Blatko1/num-word-conv/releases/) or you can just build it manually.

First, clone the source code and enter it's directory.

Then build the *executable*:

```console
$ cargo build --release
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
    │   |   **num-word-converter.exe**  <--
    │   |   ...
    │   ...
```

Copy the highlighted '*.exe*' to some more accessible place and rename it how you want.

## Usage

Open your terminal in the same directory where the '*.exe*' is and type:

```console
$ converter 435
```

> **The command name which triggers this function can be whatever you name your '*.exe*' file (*\<name_of_executable> \<any_number>*)**.

Your output should look like this:

```console
Konvertirano:
        četiristo trideset pet
```

# Converter

A BL2 modding utility. Converts from multiple individual patch files to a hotfix.

Note that it is rough around the edges and built to exactly what I needed at the time, it might not fulfill everyone's needs, but it should allow people to get started working on hotfixes. You should definitely not use this as an example of good coding practice in Rust, I spent very little time architecting this and just built what worked.

## Compilation

    cargo build

This project was built for Rust version 1.17 Beta in April 2017. While this is the version used to build the project, earlier and later versions will likely work fine. (No promises)

To install Rust, go to https://www.rust-lang.org/en-US/

## Usage

    cargo run -- <options>

Run through cargo or the executable itself.

Command line options:

    BL2_Converter 1.1
    Converts from *.hotfix files to executable BL2 console commands

    USAGE:
        converter [OPTIONS] [FILE]...

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -o, --output <OUTPUT>    File to output to, defaults to 'hotfix_output.txt'

    ARGS:
        <FILE>...    Files to convert, if empty will take all files named
                     '*.hotfix' in the current folder

Example:

    cargo run -- base_game.hotfix -o base_game.txt

    converter base_game.hotfix -o base_game.txt

## Syntax

- start \<type\> \<package?\> : Specifies the key and possibly package for the following hotfixes. Applies until the next start command.
- set \<object\> \<path\> \<value\> : Sets the path inside the object to the value. Much like the set command, but with the ability to reference individual elements in an array rather than replacing the entire array at once.
- set_cmp \<object\> \<path\> \<compare_value\> \<value\> : Like set, but compares the value to compare_value before setting it to value. (Not confirmed, but it was in the original patches so I left it in. Most of the time you can just use set)

## Types

- \<type\> : Can be one of OnDemand, Level, or Patch, corresponding to the keys SparkOnDemandPatchEntry, SparkLevelPatchEntry, and SparkPatchEntry respectively.
- \<package?\> : Optional package, used for Level and OnDemand, specifies the hotfix to be executed when that package is loaded or reloaded.
- \<object\> : An individual object, same syntax as the first element in a set command.
- \<path\> : Path inside the object to the variable being set, similar syntax to the second element in a set command, but with additional features such as array indexing using square brackets. (See examples)
- \<compare_value\> : The value to compare to, same syntax as the third element in a set command.
- \<value\> : The value to set, same syntax as the third element in a set command.

## Examples

Given are the original Gearbox hotfixes as of April 7th 2017, in both the original set format and the patch format usable with this project.

## Support

This project is unsupported, use at your own risk. Note that the parser is extremely picky and will crash if you have a syntax error.

## License

This project is licensed under the Apache V2, included in LICENSE-2.0.txt.

# rt3conf

The rt3conf program is an **unofficial** tool for configuring Railroad Tycoon 3.
With it you can run the game at modern screen resolutions that are not possible
to select in the settings. It can also be used to turn off settings that crash
the game to the point where you cannot open the game to undo the setting.

_Railroad Tycoon is trademarked by Take-Two Interactive Software, Inc., and this
software is not affiliated with them in any way._

## Usage

Open a terminal (cmd.exe on Windows) and navigate to where you want the
`engine.cfg` file to be generated. Enter the following and press enter to get a
configuration file with resolution set to 800 x 600 and hardware texture and
lighting turned off:

```console
C:\path-you-choose>rt3conf
```

Note that hardware texture and lighting is turned off by default because it can
cause a crash on newer machines with Nvidia graphics cards.

The `engine.cfg` file should then be copied to where Railroad Tycoon 3 stores
the configuration files, e.g.
`C:\Program Files (x86)\Steam\steamapps\common\Railroad Tycoon 3\Data\Configuration`
for Steam and
`C:\Program Files (x86)\GOG Galaxy\Games\Railroad Tycoon 3\Data\Configuration`
for GOG.

You can even generate a configuration file with resolutions that cannot be
chosen in the settings. For example, enter the following to generate a
configuration file with the screen resolution of 1920 x 1200:

```console
C:\path-you-choose>rt3conf -x 1920 -y 1200
```

Note that the settings will say 640 x 480. It is often possible to verify that
your monitor is running at the correct resolution by pressing one of the buttons
on your monitor.

You can see all the available options by using the `-h` or `--help` option.

## Installation

You can download rt3conf from the
[Release page](https://github.com/MichaelMcDonnell/rt3conf/releases).

The easiest way to use the executable is to place it in the same folder as where
you want the `engine.cfg` file generated. Otherwise you have to put it somewhere
that is in your `PATH` environment variable.

You can alternatively enter `cargo install rt3conf` in the terminal if you have
the [Rust](https://www.rust-lang.org/) programming language installed. Then the
executable is placed in a directory that is part of the `PATH` environment
variable, and you can run it from any directory.

## Building

If you want to build `rt3conf` from the source code then you first need to
install the [Rust](https://www.rust-lang.org/) programming language. Then
download the source code from GitHub or clone it with git. Finally open a
terminal and use the `cargo` build command to build the source code:

```console
c:\src\rt3conf>cargo build --release
```

The binary is placed in the directory `target/release`.

You can get Cargo to build and install the binary in a single step with the
`install` command, for example:

```console
c:\src\rt3conf>cargo install --path .
```

The rt3conf binary is now added to the `PATH` and can be executed from anywhere.

## Reverse Engineering Fields

The configuration files contain fields that control the configuration settings.
The configuration files are binary so there is no explanation of what each
field does. The function of each field was reversed engineered by changing a
setting in the game and then observing the changes to the files. I used
[wxHexEditor](https://www.wxhexeditor.org/) to find the changes. It has a
feature to compare the contents of two binary files.

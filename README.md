# rt3conf

An *unofficial* tool for configuring Railroad Tycoon 3.

Railroad Tycoon is trademarked by Take-Two Interactive Software, Inc., and this
software is not affiliated with them in any way.

## Usage

Open a terminal (cmd.exe on Windows) and navigate to where you want the
`engine.cfg` file to be generated. Enter the following and press enter to get a
configuration file with resolution set to 800 x 600 and hardware texture and
lighting turned off:

```console
C:\path-you-choose>rt3conf -x 1920 -y 1080
```

Note that hardware texture and lighting is turned off by default because it can
cause a crash on newer machines with nVidia graphics cards.

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

## Reverse Engineering Fields

The configuration files contain fields that control the configuration settings.
The configuration files are binary so there is no explanation of what each
field does. The function of each field was reversed engineered by changing a
setting in the game and then observing the changes to the files. I used
[wxHexEditor](https://www.wxhexeditor.org/) to find the changes. It has a
feature to compare the contents of two binary files.

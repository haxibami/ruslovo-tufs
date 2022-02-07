# ruslovo-tufs

## What's this?

ruslovo-tufs is a command-line helper tool for searching [Russian Dictionary](http://cblle.tufs.ac.jp/dic/ru/) hosted by TUFS (Tokyo Univ. of Foreign Studies). This tool fetches HTMLs from the site and parses them into a simple format. So it accesses the server just like normal browsers, but there's no warranty. For details, see LICENSE.txt bundled.

Its name originated from russkiy (русский, "Russian") + slovo (слово, "word") + Rust language.

## Usage

```
# show help
ruslovo --help

# search a word:
ruslovo русский

# specify searching method:
# prefix match (1), suffix match (2), exact match (3), fuzzy (4). Default value is 4.
ruslovo слово -s 3
```

## Sample output

![Preview](https://raw.githubusercontent.com/haxibami/ruslovo-tufs/master/preview.png)

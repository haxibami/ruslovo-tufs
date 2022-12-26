# ruslovo-tufs

## What's this?

ruslovo-tufs (`rusl`) は、東京外国語大学が提供する[オンラインロシア語辞書](http://cblle.tufs.ac.jp/dic/ru/)を検索するための非公式 CLI ツールです。検索結果の HTML を取得し、取り出したデータを表形式にして表示します。

名前の由来はロシア語 (русский) と言葉 (слово)、そして実装言語 (Rust) です。

ruslovo-tufs (`rusl`) is an inofficial CLI tool to search [Russian Dictionary](http://cblle.tufs.ac.jp/dic/ru/) hosted by TUFS (Tokyo Univ. of Foreign Studies). This tool fetches HTMLs from the site and parses them into a table format.

Its name originated from russkiy (русский, "Russian") + slovo (слово, "word") / Rust language.

## Usage

```
# ヘルプを表示
rusl --help

# 単語を検索
rusl русский

# 検索方法を指定
# 前方一致 (1), 後方一致 (2), 完全一致 (3), あいまい (4)。デフォルトの値は4です。
rusl слово -s 3
```

## Sample output

![Preview](https://raw.githubusercontent.com/haxibami/ruslovo-tufs/master/preview.png)

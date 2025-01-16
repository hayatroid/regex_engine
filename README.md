# regex_engine

[ゼロから学ぶ Rust](https://github.com/ytakano/rust_zero) の第 6 章の実装です．`()|+*?` に対応しています．

## Usage

たとえば `cargo run "foo|bar" /path/to/file` をすると，

```
expr: foo|bar
AST: Or(Seq([Char('f'), Char('o'), Char('o')]), Seq([Char('b'), Char('a'), Char('r')]))

code:
0000: split 0001, 0005
0001: char f
0002: char o
0003: char o
0004: jump 0008
0005: char b
0006: char a
0007: char r
0008: match
```

に続いて `/path/to/file` から `foo` または `bar` を含む行を列挙します。

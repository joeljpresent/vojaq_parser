# vojaq_parser

`vojaq_parser` is a Rust crate which can parse Vojaq files.

``` vojaq
ありがとう {arigatou} thank you
馬鹿 {baka} idiot | stupid
洗い熊 | アライグマ {araiguma} raccoon {animal}
サントノーレ {santonôre} Saint-Honoré {pastry | cake} French
```

## Disclaimer

This crate is very unstable and far from being finished.
You probably should not use this crate at the moment.

## Vojaq syntax

A variant is a character string which does not contain unescaped `{`, `}`, `|` or a linefeed.

A field is an array of variants separated by a `|`.

A line is an array of fields in such a way that one in two fields is enclosed between `{` and `}`.

A set is an array of lines separated by a linefeed (LF).

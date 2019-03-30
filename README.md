# vojaq_parser

`vojaq_parser` is a Rust crate which can parse Vojaq files.

``` vojaq
ありがとう {arigatou} thank you
馬鹿 {baka} idiot
アライグマ {araiguma} raccoon
サントノーレ {santonôre} Saint-Honoré
```

## Vojaq syntax

Vojaq trios is made up of three strings (or "testos") called "primo", "secondo" and "terzo".
A vojaq line describes a vojaq trio with the following syntax:
``` vojaq
PRIMO {SECONDO} TERZO
```
A Vojaq file is made up of Vojaq lines separated by a linefeed.
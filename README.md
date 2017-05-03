## Treasury CSV utils

Utils for working with published CSV files that claim to be in
HM Treasury format.

### Currently:

* Determines the encoding of the file (requires ```file```)
* Locates the header rows (naively)

### TODO:

* Convert non-UTF8 to UTF8
* Work out types for each column
* Enforce (fix) types for each column
* Generate a clean, utf8 encoded CSV.
* Walk a directory if provided arg is not a file.

### Notes

The format is described at [National Archive](http://webarchive.nationalarchives.gov.uk/20130129110402/http://www.hm-treasury.gov.uk/psr_transparency_index.htm)

Try lib-iconv bindings at https://github.com/andelf/rust-iconv

Check out libmagic bindings https://github.com/thestinger/rust-magic/blob/master/magic.rs or https://github.com/robo9k/rust-magic-sys

### Requires

```find```
```iconv```

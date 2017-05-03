## Treasury CSV utils

Utils for working with published CSV files that claim to be in
HM Treasury format.

Currently:

    * Determines the encoding of the file (requires ```file```)
    * Locates the header rows (naively)

TODO:

    * Convert non-UTF8 to UTF8
    * Work out types for each column
    * Enforce (fix) types for each column
    * Generate a clean, utf8 encoded CSV.
    * Walk a directory if provided arg is not a file.

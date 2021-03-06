# knitting_parser

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://github.com/jamesmahler/knitting_parser/workflows/Rust/badge.svg?branch=master)
[![Crates.io Version](https://img.shields.io/crates/v/knitting_parse.svg)](https://crates.io/crates/knitting_parse)

A library designed to assist with parsing knitting patterns.

There seems to be no real standard for the syntax of a knitting pattern, so we are going to do our best at describing our own.

## Syntax

- All stitch names are lower case
- A comma `,` separates stitches
- Stitches can be grouped in parenthesis `(` and `)`
- Multipliers can be provided after a group or stitch: `k x12` for twelve knits in a row
- Groups can be embedded: `(k, (p, ml)x2)x2` to produce `k, p, ml, p, ml, k, p, ml, p, ml`
- Line starting with ## are for options
	- in_round : used to say the pattern is in the round.
	- start_wrong_side : used to say the pattern starts on the wrong side.
	- first_line=X : used to say the first line is X
- Lines starting with # are ignored

## Stitches

- 1lcf
- 1rcb
- 2lcf
- 2rcb
- 3lcf
- 3rcb
- 4lcf
- 4rcb
- bead
- bo
- bobble
- k
- k2tog
- kbf
- kfb
- ktbl
- m kwise
- m pwise
- ml
- mr
- nostitch
- p
- p2tog
- pbf
- pfb
- ptbl
- sl kwise
- sl pwise
- ssk
- ssp
- yo

## Documentation
[docs.rs](https://docs.rs/knitting_parse)

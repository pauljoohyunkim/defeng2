# defeng2

`defeng2` is a wordlist generator that attempts to generate words that "look like English words", hopefully.

This is a rewrite of [defeng](https://github.com/pauljoohyunkim/defeng), originally written in C, now in rust with a bug fix.

## Build

With cargo, you can do,
```bash
cargo build --release
```

This will generate `target/release/defeng` binary.

## Usage

```
Usage: defeng2 [OPTIONS] --consonant-former-file <CONSONANT_FORMER_FILE> --vowel-file <VOWEL_FILE> --consonant-latter-file <CONSONANT_LATTER_FILE>

Options:
  -c, --consonant-former-file <CONSONANT_FORMER_FILE>  
  -v, --vowel-file <VOWEL_FILE>                        
  -C, --consonant-latter-file <CONSONANT_LATTER_FILE>  
  -m, --min <MIN>                                      [default: 2]
  -M, --max <MAX>                                      [default: 4]
  -o, --output <OUTPUT>                                [default: ""]
  -h, --help                                           Print help
```

```bash
./target/release/defeng2 -c data/c_former.txt -C data/c_latter.txt -v data/v.txt -m 3 -M 4 -o wordlist.txt
```
(You can find `austria` in this wordlist.txt)

### Warning
Duplicates are possible! (This may be due to the fact that former consonant cluster and latter consonant cluster can merge in many ways to create the same cluster.)

## How it works
defeng2 works by generating words of the following syntax

* cv(C)cv(C)...
* v(C)cv(C)cv(C)...

where c refers to a "former consonant cluster", v refers to a vowel or a diphthong, and (C) refers to optional "latter consonant cluster".

By default, consonant clusters include, but not limited to

    b, c, ...
    ch, sh, sch, ...
    bl, sl, sk, ...

## Note

Note that you do not have to set the length to be more than 5 is most cases! For example, the word strength will require only four spaces with this generator. (str-e-ng-th)

## Conception
The idea behind this was that, many words have limited number of consonants or vowels in a row. This fact applies in some languages as well, such as French, Spanish, Korean, etc.

By only outputting words that are more "word-like", one could greatly reduce the search space for penetration testing.

For analysis of this conjecture, one may look more into asymptotic equipartition property (AEP), I guess?

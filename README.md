pwrs
====

A pwgen'esque utility that generates passphrases instead of passwords.

    Usage: pwrs [options]

    Options:
        -h --help           Print this help
        -m --min MIN        Minimum number of characters in words
        -M --max MAX        Maximum number of characters in words
        -c --count COUNT    Count of passphrases to print
        -n --number NUMBER  Number of words to include in passphrases

Optionally, you can export `PWRS_WORDLIST`, by default it will use `/usr/share/dict/words`.

## Installing

    cargo build --release
    cp target/release/pwrs /usr/local/bin

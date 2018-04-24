VoLe: Flashcards CLI
====================

**Vo**cabulary **Le**arn is a command line tool for flashcards learning. It is
written in Rust.

This project has the following goals:

* Small and hackable CLI for flashcard learning.
* Efficient tool for learning foreign language vocabulary and similar.



Alternatives
------------

There are multiple alternative programs for flashcard learning. Consider the
following alternatives if you are looking for something more mature, stable,
feature rich or with GUI.

* [Mnemosyne](https://mnemosyne-proj.org/)
* [Anki](https://apps.ankiweb.net/)

SuperMemo 2
-----------

Cards repetition scheduling is based on
[SM2](https://www.supermemo.com/english/ol/sm2.htm) algorithm.

Examples
--------

Learning:

```
$ vole learn
Q: flashcard
Show answer [y, q, ?]? y
A: a card containing a small amount of information
How difficult was it [0, 1, 2, 3, 4, 5, ?]? ?
0 - complete blackout
1 - incorrect response; the correct one remembered
2 - incorrect response; where the correct one seemed easy to recall
3 - correct response recalled with serious difficulty
4 - correct response after a hesitation
5 - perfect response
? - help
How difficult was it [0, 1, 2, 3, 4, 5, ?]? 5
Continue with another card [y, q, ?]? y
...
```

Adding new card:

```bash
$ vole add learning "the acquisition of knowledge or skills"
```

Building
--------

Before you begin make sure you have installed
[Rust](https://www.rust-lang.org/en-US/install.html).

```bash
git clone git@github.com:Indy2222/vole.git
cd vole
cargo build --release
```

Binary end up in `/target/release` subdirectory.

Contributing
------------

VoLe is licensed under GPL3

I am new to Rust so I will appreciate any help or feedback. Feel free to
suggest changes or open a pull request.

You may contact me at `martin.indra at mgn dot cz`.

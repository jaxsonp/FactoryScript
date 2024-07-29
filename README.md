# FactoryScript 🏭

[![Cargo tests](https://github.com/jaxsonp/FactoryScript/actions/workflows/rust.yml/badge.svg)](https://github.com/jaxsonp/FactoryScript/actions/workflows/rust.yml)
[![Documentation Status](https://readthedocs.org/projects/factoryscript/badge/?version=latest)](https://factoryscript.readthedocs.io/en/latest/?badge=latest)

The world's number one M.O.P. (Manufacturing Oriented Programming) language, FactoryScript is a dynamically typed, interpreted programming language inspired by the magnificence of the industrial revolution.

This repository contains:

- `core/`: Cargo package containing core definitions and functions
- `docs/`: Documentation files
- `examples/`: Directory containing some FactoryScript code examples
- `interpreter/`: Cargo package containing the canonical FactoryScript Interpreter

### Table of Contents

- [Language Overview](#language-overview)
  - [Syntax](#syntax)
  - [Examples](#examples)
- [Build Instructions](#build-instructions)

## Language Overview

FactoryScript was motivated by the elegance and efficiency of factories and the modern manufacturing process. FactoryScript code describes the layout of a factory, defining various stations and interconnecting conveyor belts. Instead of variables like you'd find in typical programming languages, FactoryScript stores chunks of data in pallets, which are moved around by conveyor belts and operated on by stations, similar to functions in other languages.

For the complete reference, check out the [full documentation](https://factoryscript.readthedocs.io/en/latest/).

### Syntax

**Stations:** In general, stations are defined with square brackets, and contain a single ASCII, non-whitespace identifier, such as `[println]`, `[>=]`, or `[exit]`. There is one exception to this syntax, assign stations. They are defined with curly brackets, and contain literals to be assigned to pallets. Examples include `{"abc"}`, `{true}`, and `{4.025}`

**Conveyor Belts:** Conveyor belts are represented using contiguous Unicode [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_characters). Conveyor belts are omni-directional, but must be attached on both ends to a station. The beginning end of a conveyor belt is drawn with double line characters (`║`, `═`, `╗`, etc) while the rest of the belt is drawn with single line characters (`│`, `─`, `┐`, etc).

Text that is not a station or a conveyor belt is treated as a comment, being ignored by the interpreter. Below is an annotated hello world program.

```text
spawns an empty   assigns it the string
pallet            literal "hello world"
  v                v
[start]═─{"hello world"}═─[println]
                            ^
                        prints the pallets value
```

```sh
$ factory examples/hello_world.factory
hello world
$
```

However, because FactoryScript is unopinionated about layout, it is possible to reverse the order...

```text
[println]─═{"hello world"}─═[start]
```

... or even make the conveyor belts as unnecessarily convoluted as you want (this does not affect runtime performance).

```text
[start]═─{"hello world"} [println]
┌────────╝               └───────────────────────────────────┐
│  ┌┐  ┌┐       ┌┐ ┌┐          ┌┐  ┌┐              ┌┐    ┌┐  │
│  ││  ││ ┌───┐ ││ ││          ││  ││        ┌┐    ││    ││  │
│  │└──┘│ │ # │ ││ ││ ┌────┐   ││┌┐││ ┌────┐ │└──┐ ││ ┌──┘│  │
│  │┌──┐│ │┌──┘ ││ ││ │ /\ │   ││││││ │ /\ │ │┌─┐│ ││ │| |│  │
│  ││  ││ │└──┐ ││ ││ │ \/ │   │└┘└┘│ │ \/ │ ││ └┘ ││ │|_|│  │
│  ││  └┘ └┐┌─┘ ││ ││ └┐┌──┘   └┐┌──┘ └┐┌──┘ ││    ││ └┐┌─┘  │
└──┘└──────┘└───┘└─┘└──┘└───────┘└─────┘└────┘└────┘└──┘└────┘
```

### Examples

#### Simple loop

```text
[start]      ┌──═[and]─┐
     ╚──{1}  │   ╔─┘   ║
         ╚──[]═─[++]═─[>=]
 [println]──╝╚──{10}═──┘
```

```sh
$ factory examples/for_loop.factory
1
2
3
4
5
6
7
8
9
10
$
```

#### Greeting

```text
[start]═─{"What is your name? "}═─┐
             ╚─[print]    ╚─┐     │
                       {"Hello "} │
                          ┌─╝     │
           [print]─═[]──═[+]─═[readln]
   [println]─═{'!'}─╝
```

```sh
$ factory examples/greet.factory
What is your name? Jeff
Hello Jeff!
$
```

#### Fizzbuzz

```text
[start]═───{1}═─[]─═[gate]──═[++]───╗
                ║      └──═[<]────═[gate]────────────┐
                │           └═{100}─╝│               │
                │                    │               │
                │╔──────────────────┐║               │
       ┌──═{0}─═[]         ┌──═{0}─═[]═─[gate]       │
 [X]─═[=]─═[%]──╝║   [X]─═[=]─═[%]──╝║   │╚─[print]  │
  ║    ║    └═{3}┘    ║    ║    └═{5}┘   │           │
  │    └────────┐     │    └────────┐    │           │
 {"fizz"}       │    {"buzz"}       │    │           │
  ╚─[print]     │     ╚─[print]     │    ║           │
                └─────────────────[or]═─[!]═─{true}═─┘
                                        {'\n'}─╝
                                    [print]─╝
```

```sh
$ factory examples/fizzbuzz.factory
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz

...

fizzbuzz
91
92
fizz
94
buzz
fizz
97
98
fizz
buzz
$
```

#### First 40 fibonacci numbers

```
                       ┌═[++]───╗
  [start]═─{1}═───────[]═─────[gate]
    ║  ║              ║╚───[!=]═┘
    │  │   ┌[println] └{40}═┘║
  ╔{0}{1}  ║                 │
  │ ║  ╚──[]═───[gate]───────┘
  │ │      └───╗  ║║
  │ └─────[]═─[+]─┘│
[println]  └───────┘
```

```sh
$ factory examples/fibonacci.factory
0
1
1
2
3
5
8
13
21
34
55
89

...

14930352
24157817
39088169
63245986
102334155
$
```

More examples can be found in the `examples/` directory.

## Build Instructions

Requires Git and Cargo. First clone and cd into the repository:

```sh
git clone https://github.com/jaxsonp/FactoryScript.git && cd FactoryScript/
```

### To build interpreter:

```sh
cargo build --release --bin factory
```

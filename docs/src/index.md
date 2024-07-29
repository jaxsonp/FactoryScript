---
title: Language Reference
---

# FactoryScript Documentation

Welcome to the offical FactoryScript documentation! FactoryScript is the world's number one M.O.P. (Manufacturing Oriented Programming) language. It is a dynamically typed, interpreted programming language inspired by the beauty of the industrial revolution.

Below is an overview of the FactoryScript programming language, to find information about all the built-in station types and their functionality, go to the [station reference](station-ref.md)

# Language Overview

At it's core, FactoryScript has three primary concepts. [**Pallets**](#pallets), which contain data, which are moved around by [**conveyor belts**](#conveyor-belts) to different [**stations**](#stations) which perform operations on the pallets. A FactoryScript program defines stations and the graph of inter-connections via conveyor belts. The FactoryScript interpreter executes the program by moving pallets between stations and performing the stations' functionalities. Next, this guide will explain the three primary concepts in detail.

## Pallets

Pallets are the units of information that are being operated on in a FactoryScript program, and can be thought of as FactoryScript's "variables". Every pallet has one of six different pallet types. The following table shows the types and their properties:

| Type      | Stored data type                                            |
| :-------- | :---------------------------------------------------------- |
| Empty     | None                                                        |
| Boolean   | A binary value: `true`, `false`                             |
| Character | A single unicode character: `'a'`, `'5'`, `'🐈'`, `'\\t'`   |
| String    | A string of unicode characters: `"abc"`, `"hi\nmom!"`, `""` |
| Integer   | A 64-bit signed integer: `3`, `-15`, `596104171`            |
| Float     | A 64-bit floating point number: `2.5`, `100f`, `0.16348`    |

## Conveyor Belts

Conveyor belts are the vehicle by which pallets move to and from stations. They define how pallets move to and from different stations in a FactoryScript program.

### Syntax

Conveyor belts are represented in FactoryScript code using Unicode [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_characters). Specifically, acceptable characters are `─│└┌┐┘` (single belts), and their double variants `═║╚╔╗╝` (double belts). Conveyor belts are represented with contiguous paths made up of single belts, with the starting end of the path being marked with one double belt. For example, a conveyor belt moving pallets from A to B might be represented simply as:

```
[A]═──────[B]
```

Or in another layout:

```
     [A]
      ║
      │
[B]───┘
```

Conveyor belt length does not affect runtime performance, so conveyor belt layouts can be as convoluted as you want:

```
┌─────┐
│┌───┐│
││[A]╝│┌──────[B]
│└────┘│
└──────┘
```

## Stations

Stations are the components that perform certain actions with pallets, and can be thought of as FactoryScript's "functions". Stations are a quite dense concept, so here you will find a description of stations as a concept and how they work. To find information about the built-in station types and their functionality, go to the [station reference](station-ref.md)

### Syntax

Stations are defined with square brackets\*, with an ASCII, non-whitespace identifier string between. For example, `[start]`, `[++]`, and `[println]` are valid stations in FactoryScript code, however `[print ln]`, `(++)`, and `[日本語]` are invalid.

:::note

Assign stations are an exception, see [Special Stations](#special-stations)\_

### Behavior

Every station takes in

### Special Stations

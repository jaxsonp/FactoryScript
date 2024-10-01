
.. toctree::
   :maxdepth: 3
   :hidden:

   Language Documentation <self>
   Station Reference <station-ref>

====================================
FactoryScript Language Documentation 
====================================

Welcome to the offical FactoryScript documentation! FactoryScript is the world's number one M.O.P. (Manufacturing Oriented Programming) language. It is a dynamically typed, interpreted programming language inspired by the grandeur of the industrial revolution.

Below is an overview of the FactoryScript programming language, to find information about all the built-in station types and their operations, go to the :doc:`station reference<station-ref>`.

Language Overview
-----------------

In a nutshell, FactoryScript consists of :ref:`pallets<Pallets>`, which contain data, that are moved around the program by :ref:`conveyor belts<Conveyor Belts>` to different :ref:`stations<Stations>` which perform operations on the pallets. A FactoryScript program defines stations and the graph of inter-station connections via conveyor belts. The FactoryScript interpreter executes the program by moving pallets between stations and executing the stations' operations. In the following sections, this guide will explain the three primary concepts in detail.

Pallets
-------

Pallets are the units of information that are being operated on in a FactoryScript program, and can be thought of as FactoryScript's "variables". Every pallet has one of six different pallet types. The following table shows the types and their properties:

========= =============================================================
Type      Stored data type                                         
--------- -------------------------------------------------------------
Empty     None                                                        
Boolean   A binary value: ``true``, ``false``                             
Character A single unicode character: ``'a'``, ``'5'``, ``'🐈'``, ``'\t'``  
String    A string of unicode characters: ``"abc"``, ``"hi\nmom!"``, ``""`` 
Integer   A 64-bit signed integer: ``3``, ``-15``, ``596104171``            
Float     A 64-bit floating point number: ``2.5``, ``100f``, ``0.16348``  
========= =============================================================

Conveyor Belts
--------------

Conveyor belts are the vehicle by which pallets move to and from stations. They define how pallets move to and from different stations in a FactoryScript program.

Conveyor belts are represented in FactoryScript code using `Unicode box-drawing characters <https://en.wikipedia.org/wiki/Box-drawing_characters>`_. Specifically, acceptable characters are ``─│└┌┐┘`` (single belts), and their double variants ``═║╚╔╗╝`` (double belts). Conveyor belts are represented with contiguous paths made up of single belts, with the starting end of the path being marked with one double belt. For example, a conveyor belt moving pallets from A to B might be represented simply as: ::

   [A]═─────[B]


Or any other layout you may need (or want): ::
   
   [B]─────═[A]            [A]═────────┐
                             ┌─────────┘
        [A]    ┌─────┐       └─────────┐
         ║     │┌───┐│       ┌─────────┘ 
         │     ││[A]╝│[B]    └─────────┐
   [B]───┘     │└────┘│      ┌─────────┘
               └──────┘      └─────────[B]

Conveyor belt length does not affect runtime performance, so conveyor belt layouts can be as convoluted or as simple as the programmer wants.

Conveyor belts must be connected to a station on both ends. A conveyor belt end is considered connected simply if it points into any character of a station.

Stations
--------

Stations are the components that perform certain actions with pallets, and can be thought of as FactoryScript's "functions". This section of the documentation is an outline of stations as a concept and how they work. To find information about the built-in station types and their functionality, go to the :doc:`station reference<station-ref>`.

Syntax
^^^^^^

Stations are defined with square brackets*, with an ASCII, non-whitespace identifier string between. For example, ``[start]``, ``[++]``, and ``[println]`` are valid stations in FactoryScript code, however ``[print ln]``, ``(++)``, and ``[日本語]`` are invalid.

\* *Assign stations do not use square brackets, see* :ref:`Special Stations`

Behavior
^^^^^^^^

A station can have any number of conveyor belt inputs, also known as bays. Every station type has a defined number of bays that need to be occupied before that station's operation is triggered. When an operation is triggered, the station consumes *all* of the pallets in it's bays. Depending on its type, the station might then produce an output pallet that is sent out to all its output conveyor belts, which start with a double belt symbol (``═``, ``║``, ``╝``, etc). 

Let us use the addition station ``[+]`` as an example. By its definition, this station requires 2 inputs (the two operands), and it does produce an output pallet (the sum). That means that once this station has two occupied bays, it will consume the pallets in its bays, then produce a new pallet containing the sum.

.. danger::
   If a pallet enters an already occupied bay, the original pallet **will be dropped**, being overwritten by the new pallet.


Special Stations
^^^^^^^^^^^^^^^^

There are a handful of unique stations that are exceptions to the typical station behavior, here are a few that are important to know:

Start Station
"""""""""""""

This station simply spawns an empty pallet at the beginning of program execution. It is the only station that takes in zero inputs, and only is triggered once.

Assign Station
""""""""""""""

This station simply assigns a value to a pallet, and it is an exception to typical station syntax. Normal stations are defined using square brackets (``[...]``) with an identifier inside, but these stations are identified using curly brackets (``{...}``) and contain the value to be assigned. For example, the station ``{"Hello world!"}`` will assign the string ``"Hello world!"``, the station ``{5.4}`` will assign the float ``5.4``, and the station ``{}``, will make the pallet into an empty pallet.

Joint Station
"""""""""""""

This station is often defined as a pair empty square brackets (``[]``), and is commonly used for routing pallets. Whenever it receives a pallet, it will take that pallet and simply output it on any number of outputs. This can be used to join different conveyor belt lines with one-to-many, many-to-one, and many-to-many relationships.


Bay Modifiers
^^^^^^^^^^^^^

Some station operations may be dependent upon the order of the input pallets (subtraction, for example, where ``a-b != b-a``). The order of bays are decided by where they enter the station from. The default precedence is clockwise, beginning from the northern edge. Below is a diagram showing each conveyor belt location, and its default precedence order:

== == == == ==
\  1  2  3  \  
8  [  \- ]  4
\  7  6  5  \  
== == == == ==

In order for FactoryScript to remain unopinionated about program layout, you can add modifiers to stations that alter the precedence order. You can add up to one directional modifier (``N``, ``S``, ``E`` or ``W``) that changes which edge the order starts from, and optionally the reverse modifier (``*``) which reverses the order from clockwise to counter-clockwise.

Here are some examples:

``[-:S]`` -- clockwise from south:

== == == == == == ==
\  7  8  9  10 11  \  
6  [  \- :  S  ]  12
\  5  4  3  2  1  \  
== == == == == == == 

``[-:*]`` -- counter-clockwise from north:

== == == == == == ==
\  5  4  3  2  1  \  
6  [  \- :  \* ]  12
\  7  8  9  10 11 \  
== == == == == == == 

``[-:E*]`` -- counter-clockwise from east:

== == == == == == == ==
\  7  6  5  4  3  2  \  
8  [  \- :  E  \* ]  1
\  9  10 11 12 13 14 \  
== == == == == == == == 



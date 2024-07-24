.. FactoryScript documentation master file, created by
   sphinx-quickstart on Wed Jul 24 15:58:08 2024.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

.. toctree::
   :hidden:
   :glob:

FactoryScript Documentation
===========================

The world's number one M.O.P. (Manufacturing Oriented Programming) language, FactoryScript is a dynamically typed, interpreted programming language inspired by the beauty and the elegance of the industrial revolution.


Language Overview
=================

Pallets
-------

At their core, all FactoryScript programs are simply faciliting the movement and transformation of ephemeral variables called `pallets`. The following table shows the different pallet types and their properties:

========= ================
Type      Stored Data Type
========= ================
Empty     None
Boolean   A binary value: :code:`true`, :code:`false`
Character A single character: :code:`'a'`, :code:`'5'`, :code:`'🐈'`, :code:`'\\n'`
String    A string of characters: :code:`"abc"`, :code:`"hi\\nmom!"`, :code:`""`
Integer   A 64-bit signed integer: :code:`3`, :code:`-15`, :code:`596104171`,
Float     A 64-bit floating point number: :code:`2.5`, :code:`100f`, :code:`0.16348`,
========= ================


Stations
--------

Conveyor Belts
--------------

Station Reference
=================

..
   Glossary
   ========
   .. glossary::
      Pallet
         A variable that stores a value depending on its type
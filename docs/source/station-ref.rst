
=================
Station Reference
=================

A "number pallet" is referring to either an integer or float pallet. "Matching" number pallets are referring to multiple number pallets that are of the same type, either all integer or all float.

The descriptions will sometimes use subscripted 1-indexed numbers, like this: :sub:`1`, to convey argument order when it matters. 

============= ============ =========== ======== ===========
ID            Shorthand ID # of Inputs Outputs? Description
============= ============ =========== ======== ===========
``{...}``     \            1           ✓        Assign station, accepts any pallet type and changes its value to the literal contained within the curly braces. See the language documentation for details.
``[add]``     ``[+]``      2           ✓        Addition, accepts two matching number pallets and outputs the sum OR concatenates a character or string pallet :sub:`2` onto the end of another string pallet :sub:`1`.
``[and]``     \            2           ✓        Boolean and, accepts two boolean pallets.
``[dec]``     ``[--]``     1           ✓        Decrement, accepts a number pallet and outputs its value minus one.
``[div]``     ``[/]``      2           ✓        Division, accepts two matching number pallets (dividend :sub:`1` and divisor :sub:`2`) and outputs the quotient.
``[eq]``      ``[=]``      2           ✓        Equals, returns a boolean pallet representing whether the two supplied pallets are equivalent.
``[exit]``    \            1           \        Immediately exits the program when any pallet is received.
``[filter]``  ``[X]``      1           ✓        Accepts any type of pallet and passes it through, unless it is a ``false`` boolean pallet, in which case it will drop the pallet. Useful for control flow.
``[gate]``    \            2           ✓        Requires one boolean pallet and another pallet of any type, in any order. If the boolean pallet is true, the other pallet is passed through, otherwise the other pallet is dropped. Useful for control flow.
``[gt]``      ``[>]``      2           ✓        Greater than, accepts two matching number or boolean pallets, returns a boolean pallet with the comparison result.
``[gte]``     ``[>=]``     2           ✓        Greater than or equal, accepts two matching number or boolean pallets, returns a boolean pallet with the comparison result.
``[inc]``     ``[++]``     1           ✓        Increment, accepts a number pallet and outputs its value plus one.
``[joint]``   ``[]``       1           ✓        Simply passes through any pallet it receives. Useful for control flow.
``[lt]``      ``[<]``      2           ✓        Less than, accepts two matching number or boolean pallets, returns a boolean pallet with the comparison result.
``[lte]``     ``[<=]``     2           ✓        Less than or equal, accepts two matching number or boolean pallets, returns a boolean pallet with the comparison result.
``[mod]``     ``[%]``      2           ✓        Modulo, accepts two matching number pallets and outputs the remainder of one pallet :sub:`1` divided by the other :sub:`2`.
``[mult]``    ``[*]``      2           ✓        Multiplication, accepts two matching number pallets and outputs their product.
``[ne]``      ``[!=]``     2           ✓        Not equals, returns a boolean pallet representing whether the two supplied pallets are not equivalent.
``[not]``     ``[!]``      1           ✓        Boolean not, accepts two boolean pallets.
``[or]``      \            2           ✓        Boolean and, accepts two boolean pallets.
``[print]``   \            1           \        Accepts and prints a pallet to stdout.
``[println]`` \            1           \        Accepts and prints a pallet to stdout with a newline appended.
``[readln]``  \            1           ✓        When any pallet is received, this station blocks while reading from stdin until a newline is received, outputting a string pallet containing the input received. The original pallet is dropped.
``[start]``   \            0           ✓        Marks the entry point of program execution. Spawns one empty pallet when the program starts, then becomes dormant for the rest of the program execution.
``[sub]``     ``[-]``      2           ✓        Subtraction, accepts two matching number pallets and outputs the difference (pallet :sub:`1` minus pallet :sub:`2`).
============= ============ =========== ======== ===========

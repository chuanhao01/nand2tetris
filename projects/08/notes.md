# From the book

8.2.1 Program Flow Commands
The VM language features three program flow commands:

label `label`
This command labels the current location in the function’s code.
Only labeled locations can be jumped to from other parts of the program. The scope
of the label is the function in which it is defined.
The label is an arbitrary string composed of any sequence of letters, digits, underscore (_), dot (.), and colon (:) that does not begin with a digit.

m goto label This command effects an unconditional goto operation, causing execution to continue from the location marked by the label. The jump destination must
be located in the same function.
m if-goto label This command effects a conditional goto operation. The stack’s
topmost value is popped; if the value is not zero, execution continues from the location marked by the label; otherwise, execution continues from the next command in
the program. The jump destination must be located in the same function.

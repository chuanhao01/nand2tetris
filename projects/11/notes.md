implement infinite nesting of scopes, with scoped variable names?
will have to see

## Notes
In jack and when writing the compiler
There is a distinct difference between the different subroutines
- constructor
  - Will specifically alloc memory for a new class object (of the class file)
  - also callers to the constructor function will not need to push the base address
- method
  - Will specifically set `pointer 0` to the `this` argument
  - Setup the object base address
- function
  - will do no such thing

### subroutinecall

- subroutineName(expression)
  - Should only be called within a class (so this base address should already be set)
  - just need to `push pointer 0`

### subroutine dec

#### constructor
push the amount of blocks needed for the object
`push constant 2`
call alloc with 1 arg
`call Memory.alloc 1`
then u have the base address at the top of the stack
`pop pointer 0` to set the base address

## Things not implemented (In no order)
- term/expressions
  - string constant
  - keywords constant

# Raggedy Rust Calculator

My mini-project of creating a some-what functional *(CLI)* calculator in rust.

### Current features:
- Correct Order of operations

### Eventual Goals: 
- GUI
- Support for parenthesis
- Memory/variables
- Additional functions and operations
- Perhaps solve for x functionality and more
- Deal with improving floating errors (not sure if even possbile) 
- Evaluate results in fractions or exact constants sqrt(2) (example: 0.3333 as 1/3)
- Option between precision and aproximations


## Idea Notes

Going be tackeling implementing support for parenthesis and then consequently support for unary minus, so that `(-2)^2` or `1/-2` can be properly evaluted without errors.

So far the idea is some kind of recursivity where the all content within `()` is viewed as an seperate input to be evaluted.


## Project Notes

Originally was thinking of generating a tokenized list of the input and then make some kind of an abstract syntax tree, but was not neccessary so far.
Also wanted to ommit the enum variant for subtraction, based on the property that `x - y = x + (-y)` but then i realized that for example `5-2^2` would be evaluated incorrectly as `5+(-2)^2` and decided not to.

As of the current version where only correct order of operaions is supported, the implementation works by first taking the input from the user and parsing it into a list of enums that represent either a value or some operation. This list then is iterated over repeatedly where the index of first highest "priorty" of operation is saved. It then gets the left and right value *(safe to assume in this version to always be the case)*. These then are used to calculate the value using whatever operation the index is pointing to.
The three index; left_value, operation, right_value in the `vec` of tokens are replaced with the single calculated value and this process repeats until only a single value left in the `vec` *(or returns an error)*. 



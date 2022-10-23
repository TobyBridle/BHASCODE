# The Pseudocode Compiler

**Example usages**

```
-- This is a program that prints 'hello world'
PRINT "hello world"

-- This also prints 'hello world'
PRINT("hello world")

-- This ALSO prints 'hello world'
string str = "hello world"
PRINT str

-- The brackets around functions are optional if they are only taking in one argument.
-- If there is more than one argument, or an operation happening on the one argument, then there must be surrounding braces
-- Example
foo x + 5 -- This would compile as (foo(x)) + 5
foo(x + 5) -- This would compile as foo(x+5)
```

_Things that I found useful whilst developing (in no particular order)_

- [C syntax written in BNF (courtesy of University of Manchester)](http://www.cs.man.ac.uk/~pjj/bnf/c_syntax.bnf)
- [The series on Compilers by Anita](https://www.youtube.com/channel/UCG-KXsLzjZMQaDBER5ddx0Q)
- [A series handouts and lectures for the compiler module at Stanford University](https://web.stanford.edu/class/archive/cs/cs143/cs143.1128/)
- [Context-free Grammars](https://en.wikipedia.org/wiki/Context-free_grammar)
- [Extended Backus-Naur Form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
- [This series by "Chonky Raccoon Coding" on creating a lexer and parser in Rust](https://www.youtube.com/channel/UCl_rCpOUXYvSEzfK5qyJxJQ)
- [Python EBNF](https://docs.python.org/3/reference/grammar.html)

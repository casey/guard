# guard

This crate exports a macro which implements most of [RFC 1303](https://github.com/rust-lang/rfcs/pull/1303) (a "let-else" or "guard" expression as you can find in Swift.

The syntax proposed in the RFC was `if !let PAT = EXPR { BODY }` or `let PAT = EXPR else { BODY }` (where `BODY` _must_ diverge). Due to implementation details, this macro has the rather awkward syntax `guard!({ BODY } unless EXPR => PAT)`. Alternative syntaxes may be added in the future.

## How it works

It's difficult to implement this behavior as a macro, because a `let` statement must be created in the enclosing scope. Besides that, it is desirable to avoid the necessity of repeating the identifiers bound by the pattern. The strategy used here is to scan the pattern for identifiers, and use that to construct a top-level `let` statement which internally uses a `match` to apply the pattern. This scanning is _almost_ possible -- see the limitations below.

This strategy also means that `PAT` needs to be input to the macro as an unparsed sequence of token trees. There are two ways to take an unbounded sequence of token trees as input without causing ambiguity errors: put the token trees at the end (my current choice) or enclose them in brackets. The backwards invocation syntax is a result of this choice.

There are a number of subtleties in the expansion to avoid various warning and pitfalls; see the macro source for more details.

## Limitations

1. Expressions in the pattern are _not_ supported. This is a limitation of the current Rust macro system -- I'd like to say "parse an identifier in this position, but if that fails try parsing an expression" but this is is impossible; I can only test for _specific_ identifiers. It's easy to get around this restriction: use a pattern guard (as in `match`) instead.
2. Empty, un-namespaced enum variants and structs cause the expansion to fail, because the macro thinks they are identifiers. It's possible to get around this as well, though an open PR is aiming to take away the easiest workaround:
   a. For empty enum variants, use `Empty(..)` until [#29383](rust-lang/rust#29383) lands, after that include the enum name as in `Enum::Empty`
   b. For unit-like structs, use `Empty(..)` until [#29383](rust-lang/rust#28393) lands, after that namespace it as in `namespace::Empty`, or use `Empty{}` (requires `#![feature(braced_empty_structs)]`

## Non-limitations

1. Unlike normal `match` and `if let`, `guard!` allows `PAT` to be irrefutable. This is achieved by inserting a no-op `if true == true` pattern guard in the generated match statement to eliminate the E0001 "unreachable pattern" error.


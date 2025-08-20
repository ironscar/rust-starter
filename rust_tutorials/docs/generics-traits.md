# Generics, Traits and Lifetimes

- Generics allow common implementations for different concrete types
- Traits constrain specific types to have a particular behavior
- Lifetimes specify how long a borrowed reference is valid

---

## Generics

- We specify a `<T>` beside the function name and then can refer to that type
- Some operators cannot be applied to all generic types
  - `<` or `>` can only be applied to types that implement the trait `PartialOrd`
  - thus, we have to sometimes restrict types a generic function can take
  - we can do this with `<T: Trait>` where `Trait` is name of trait
- For a struct, we specify it beside the name of the struct during declaration
  - but we don't need to specify again when creating instances of struct
- For an enum, we specify it beside the name of the enum during declaration
  - we also need to specify all types in the enum during variable declaration

---

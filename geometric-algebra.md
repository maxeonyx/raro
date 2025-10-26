# Built-in Geometric Algebra Library

A built-in geometric algebra module should embrace the expressive power of the language while
remaining approachable for new users. Key design pillars include:

## Strong Typing Across Multiple Levels

* **Module-level type safety** ensures that the geometric algebra (GA) flavor—such as Euclidean,
  projective, or conformal—is fixed at import time, preventing accidental mixing of incompatible
  signatures.
* **Grade-aware value types** distinguish scalars, vectors, bivectors, trivectors, and general
  multivectors through dedicated wrappers. Static dispatch over these types eliminates ambiguous
  coercions and exposes compile-time diagnostics when illegal products are formed.
* **Operation-level typing** encodes the valid domain and codomain of products (geometric, outer,
  inner, left/right contractions). This keeps generated code honest in generics-heavy codebases and
  clarifies intent in high-level APIs.

## Intentional, Well-Named Aliases

* **Algebra aliases** follow established abbreviations (e.g., `GA2E` for 2D Euclidean, `PGA3` for
  3D projective, `CGA5` for conformal). Names surface both dimension and metric so developers can
  quickly locate the correct module.
* **Grade aliases** expose canonical names for multivector grades—`Scalar`, `Vector`, `Bivector`,
  `Rotor`, `Translator`, `Motor`—with precise documentation describing their geometric role inside
  each algebra.
* **Operation aliases** tailor the API to the selected algebra, e.g., `reflect`, `meet`, `join`,
  or `sandwich`, while still delegating to the core geometric product machinery under the hood.

## Documentation Coupled to Interpretations

Every alias and operation documents the underlying algebra **and** the interpretive view employed:

* The **mirror / plane-based view** highlights dual representations where planes are primitive
  elements and reflections are fundamental operations. Documentation emphasizes bivector-driven
  transformations, detailing how mirrors compose and how motor algebras capture rigid motions.
* The **point-based view** centers points (or homogeneous points) as atomic elements. Guidance
  focuses on constructing lines, circles, and spheres through joins, along with intuitive examples
  of interpolating poses via rotors and translators.

Providing both interpretive lenses prevents documentation from being tied to a single intuition and
helps teams adopt the algebra that matches their problem space. With strong typing, thoughtful
aliases, and interpretation-aware documentation, the built-in module becomes a dependable and
welcoming foundation for geometric computing.

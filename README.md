# Linear-Algebra-CL
General compute on GPUs using a linear algebra API + Accelerated Linear Algebra Algorithms


## How-to Install


## Basic Usage


## Development Roadmap - Core Issues
- [ ] Define the Physical Device Database and it's API
- [ ] Stateful extension tracker, such that this state can have continuous conversations with the database
    - Needs to be abstracted such that algorithms can interface with it through an API
- [ ] Algebraic Types Defined
    - [ ] Element-types: Monoids, Groups, Rings, Fields
        - [ ] Can enable SIMD on just element types as well
    - [ ] Combination-types: Modulelites (Modules over monoids/groups), Modules (over any element type)
        - [ ] Vector spaces (derived from modules) will be extended to:
            - [ ] Matrix spaces
            - [ ] Polynomial spaces
            - [ ] Banach spaces
            - [ ] Lie algebras
- [ ] Standard Implementations on Types w/ appropriate functionality
    - [ ] Discrete & Continuous implementations (e.g. derivative)
- [ ] Dialectical/Contextual/Stateful Implementations (for optimization & ability to run without certain extensions)

## Development Roadmap - Other Issues

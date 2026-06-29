A multi-threaded simulator for CellularAutomata. Has a default implementation for Conways Game Of Life.

To try it out:
navigate to crates/cli:
`cd crates/cli`
and run cargo with a file containing the start state. test.life is a working example you can use:
` cargo run test.life`

to implement your own CellularAutomaton youll need to implement the CellularAutomaton trait. see conway.rs for an example. then you can create a new Simulation and run it. see main.rs for an example.

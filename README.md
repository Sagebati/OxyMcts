OxyMcts
===

If you don't know what is monte carlo tree search, I bet you know about AlphaGo the program that beat the best Go player in the world. 
Their approach was innovative because they used a neural network. but the neural network was not alone, the NN was "only" leading the MCTS. 
 
 For more : [Wikipedia link](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search)
 
## Presentation

Library to play with an monte carlo tree search, it's generic over the game. see `examples/tictactoe`.

This lib is also generic over the implementation and modular. indeed a normal MCTS can be divided in four operations, 
`tree policy(selection, expansion)`, `simulation`, `backprogation`. 
With this implementation we can change the simulation without changing the selection for example. Any programmed operations 
are interchangeable. So anyone can implement his `tree policy` without touching at my code and it will run. We can view this library as a
 collection of traits for the mcts and the glue. Easily extendable.

## Implementation details
This tree doesn't store the "game" state in the tree's nodes instead it stores only the historic of moves until the state. This approach can be beneficial if the "state" is an memory intensive struct. it will also helps for future parallelization.
 
 ## At the moment
  - contains basic approaches (UCT) (naive simulation)
  - not heavy tested
  - not heavy profiled, but it can do just well.
  - doesn't uses hashes in the nodes.
  - works out of the box with the UCT approach.
  
 ## TODO
 - maybe use an persistent List for the node historic.
 - use another library for managing the tree.
 - provide a parallel mcts implementation.
 - provide a mcts implementation using hashtables.
 - Use trait for abstract from the node and tree.

# Experiment
With C = 1.41421, 10000 rollouts,
        in a tictactoe of dim 6, in 1000 games versus a random bot who begins
        the mcts wins 52.4% of time, there is 41.5% nulls, so random bot wins 6.1% of the time 
        (see examples/tictactoe)
OxyMcts
===

If you don't know what is monte carlo tree search, I bet you know about AlphaGo the program that beat the best Go player in the world. 
Their approach was innovative because they used a neural network. but the neural network was not alone, the NN was "only" leading the MCTS. 
 
 For more : [Wikipedia link](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search)

Library to play with an monte carlo tree search, it's generic over the game. see `examples/tictactoe`.

This lib is also generic over the implementation and modular. indeed a normal MCTS can be divided in four operations, 
`tree policy(selection, expansion)`, `simulation`, `backprogation`. 
With this implementation we can change the simulation without changing the selection for example. Any programmed operations 
are interchangeable. So anyone can implement his `tree policy` without touching at my code and it will run.

## Implementation details
This tree doesn't store the "game" state in the tree's nodes instead it stores only the historic of moves until this state. This approach
 can be beneficial if the "state" is an memory intensive struct. it will also helps for future parallelization.
 
 
 ## At the moment
  - it's not parallel.
  - contains basic approaches (UCT)
  - not heavy tested
  - not heavy profiled, but it can do just well.
  - doesn't uses hashes in the nodes.
  
 ## TODO
 - maybe use an persistent List for the node historic

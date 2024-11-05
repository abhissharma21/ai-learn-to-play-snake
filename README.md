<img width="893" alt="image" src="https://github.com/user-attachments/assets/f9281d46-adb4-4dd4-b872-7d4dae8539df">


Algorithm
Initialization:
The simulation begins at Generation 0.
A new population of snakes is created, each with a neural network initialized with random weights and biases.
Game Update:
Each step, every game is updated by passing vision inputs to the neural network to decide the snake's action.
A game is flagged as complete if:
The snake collides with walls or itself.
The snake fails to eat food within a certain number of steps, preventing indefinite looping.
Generation Completion:
The generation continues updating each game until all games are complete.
Fitness Evaluation:
At the end of each generation, snakes are ranked based on their performance.
Parent Selection:
Parents for the next generation are chosen based on rankings. Higher-ranked snakes have a higher probability of being selected as parents.
Reproduction:
Techniques such as roulette wheel selection, elitism, and other methods are used to generate children for the next generation.
New Generation:
A new population is created, and the process repeats from step 2 until the simulation is manually stopped.
This iterative process leads to snakes fine-tuning their strategies, resulting in longer snakes over time.

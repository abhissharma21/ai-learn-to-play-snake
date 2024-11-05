<h1>AI LEARNS TO PLAY SNAKE</h1>

<img width="893" alt="image" src="https://github.com/user-attachments/assets/f9281d46-adb4-4dd4-b872-7d4dae8539df">


<h1>Algorithm</h1>
<hr>
<ol>
  <li><strong>Initialization:</strong></li>
<ul>
  <li>The simulation begins at Generation 0.</li>
  <li>A new population of snakes is created, each with a neural network initialized with random weights and biases.</li>
</ul>
<li><strong>Game Update:</strong></li>
<ul><li>Each step, every game is updated by passing vision inputs to the neural network to decide the snake's action.</li>
<li>A game is flagged as complete if:
  <ul>
  <li>The snake collides with walls or itself.</li>
  <li>The snake fails to eat food within a certain number of steps, preventing indefinite looping.</li>
  </ul>
</li>
</ul>
  
<li><strong>Generation Completion:</strong></li>
<ul>
  <li>The generation continues updating each game until all games are complete.</li>
</ul>
<li><strong>Fitness Evaluation:</strong></li>
<ul>
  <li>At the end of each generation, snakes are ranked based on their performance.</li>
</ul>
<li><strong>Parent Selection:</strong></li>
<ul>
  <li>Parents for the next generation are chosen based on rankings. Higher-ranked snakes have a higher probability of being selected as parents.</li>
</ul>
<li><strong>Reproduction:</strong></li>
<ul>
  <li>Techniques such as roulette wheel selection, elitism, and other methods are used to generate children for the next generation.</li>
</ul>
<li><strong>New Generation:</strong></li>
<ul>
  <li>A new population is created, and the process repeats from step 2 until the simulation is manually stopped.</li>
  <li>This iterative process leads to snakes fine-tuning their strategies, resulting in longer snakes over time.
</li>
</ul>
</ol>

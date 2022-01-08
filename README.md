# Evolution Simulator

## Description
- A program which simulates natural selection and evolution
- Uses neural networks to make decisions.
- Every generation there are mutations which change the behavious of the characters (arrows), an essential part of a neural net.
- Uses nodes and edges (neurons) in the neural net, instead of matrices<sup>1</sup>.
- Written in Rust with as few crates as possible<sup>2</sup>; this means no machine learning crates, or anything to help create the neurons.
- Runs on the console, with no extra window created. This is because I am writing it on [replit](https://replit.com/) which currently doesn't support GUI's for Rust. Also, why not?

## Purpose
- For me to learn Rust in more depth.
- For me to learn more about neural networks.
- Perhaps help someone else learn, however it is important for anyone doing that to know that this is my first time writing anything like this so it very possibly written terribly<sup>3</sup>.

## How it works
- Each character, called an arrow, has 5 states- up, down, left, right, and still. That is set by it's previous movement, with still being both no movement and the default.
- The "arena", which they all reside in, is some arbitrary size which wraps around when the sides are hit. If you go off the left, you'll appear on the right.
- The sides of the arena are indicated by a █ character. The edge is not included in the useable arena area.
- Various "powerups" are in the arena, indicated by either an "a", "h", or "e". They boost the arrows' speed, health, or eyesight, respectively.
- Speed lasts some<sup>4</sup> number of ticks.
- Health increases the amount of health they have by 1<sup>4</sup>. When an arrow hits another arrow head-on, they lose the amount of health the other one had, and if their health is 0 they die.
  - Attacking is done with the front of the arrow, and is the only way they can deal damage. If they get hit from the front of another arrow anywhere, even their front, the they take damage. As mentioned previously, the arrow with higher health wins and if they are the same then they both die.
  - If an arrow is hit from the sides or back they will take damage without dealing damage. Because of this, the health attribute can be thought of as both attack and health.
- When some number of arrows left reaches some number<sup>4</sup>, the round ends and the ones left are used to repopulate the next round.
- On the first iteration, all the arrows have the same default value. After that, they are all (likely mutated) copies of their parent arrow. There are always some number<sup>4</sup> of arrows which aren't mutated left over, in case all of the mutations are bad.

## Todo list
- Dumping of a trained model, likely in RON format. This can help with debugging and exposition of this project.
- Config files to change currently hardcoded values.


## Notes
<sup>1</sup>Possibly rewrite the neural network to use matrices instead.  
<sup>2</sup>Currently using `rand` and `ordered-float`.  
<sup>3</sup>Feel free to contribute!  
<sup>4</sup>Currently hard-coded.

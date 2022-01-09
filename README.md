# Evolution Simulator

## Description
- A program which simulates natural selection and evolution
- Uses neural networks to make decisions.
- Every generation there are mutations which change the behavious of the characters (arrows), an essential part of a neural net.
- Uses matrices instead of nodes and edges for the neural net.
- Written in Rust with as few crates as possible<sup>1</sup>; this means no machine learning crates, or anything to help create the "neurons".
- Runs on the console, with no extra window created. This is because I am writing it on [replit](https://replit.com/) which currently doesn't support GUI's for Rust. Also, why not?

## Purpose
- For me to learn Rust in more depth.
- For me to learn more about neural networks.
- Perhaps help someone else learn, however it is important for anyone doing that to know that this is my first time writing anything like this so it very possibly written terribly<sup>2</sup>.

## How it works
- Each character, called an arrow, has 5 states- up, down, left, right, and still. That is set by it's previous movement, with still being both no movement and the default.
- The "arena", which they all reside in, is some arbitrary size which wraps around when the sides are hit. If you go off the left, you'll appear on the right.
- The sides of the arena are indicated by a █ character. The edge is not included in the useable arena area.
- Various "powerups" are in the arena, indicated by either an "a", "h", or "e". They boost the arrows' speed, health, or eyesight, respectively.
- Speed lasts some<sup>3</sup> number of ticks.
- Health increases the amount of health they have by 1<sup>3</sup>. When an arrow collides with another arrow, they both lose the amount of health the other one had. If their health is 0, they die.
- When some number of arrows left reaches some number<sup>3</sup>, the round ends and the ones left are used to repopulate the next round.
- On the first iteration, all the arrows have the same default value. After that, they are all (likely mutated) copies of their parent arrow. There are always some number<sup>3</sup> of arrows which aren't mutated left over, in case all of the mutations are bad.

## Todo list
- Dumping of a trained model, likely in RON format. This can help with debugging and exposition of this project.
- Config files to change currently hardcoded values.

## How to build
- Install Rust and Cargo, preferably using [rustup](https://rustup.rs/).
- Navigate to wherever you cloned this repository to, right above `/src/`.
- Run `cargo run`.


## Notes
<sup>1</sup>Currently using `rand` and `ordered-float`.  
<sup>2</sup>Feel free to contribute!  
<sup>3</sup>Currently hard-coded.

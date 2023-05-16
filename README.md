# Interview Option 1

This is the first option for the AF interview challenge. You are asked to develop a program that controls a simple simulation. The goal is to get the included vehicle to hover. The simulation uses very simple 1D kinematics. Your vehicle has a maximum thrust of 20N, and your craft weighs 1KG. You may not change `sim/src/lib.rs`. Ideally, you will write a simple [control loop](https://en.wikipedia.org/wiki/Control_loop). Control loops generally take information about the current state of a system, they calculate a desired change to the system and then command actuators (thrusters, motors, etc) to change the system. The classic example of a control loop is a [PID loop](https://en.wikipedia.org/wiki/PID_controller). If you aren't comfortable writing a control loop, try out the second option instead.

Ask any questions you want, but only some will be answered. 

### Requirments
- The craft must be able to hover within a reasonable error, you can decide what that means.
  - In this bounded test; Reasonable error was to maintain utmost stability with minimal osicillatory modes; Due to too many unknowns it's a bit out of reason to continue fine tuning precision.
- You must document what you are doing in the source code with comments, and be prepared to explain your design, tradeoffs, and whatnot.
  - The code is commented and attemtping to be as self explanatory as possible.
- You must include a way of visualizing the vehicle, this could be a graph, an animation, or anything you can think of.
  - I built this out of the provided `const_throttle` example, upon running that example a web browswer should open providing a plotly plot of the resulting simulation
- Ensure you use Git as you would in a real project. Commit often, we are here to see your process.
- Please outline how you tested your code. This could be manual testing, automatic testing, or something in-between.
  - This code was tested in the loop of an integration example manually; Tuning was also done manually via observation of the plots and years of anecdotal experience, along with simple bracketing until performance was reasonable. Tuning began with P only then PD, then PID terms.

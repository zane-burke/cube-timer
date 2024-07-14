# cube-timer
A simple program for speedcubing made using the Iced framework.

## Features

- Separate inspection and solve timers.
- A display that shows the 5 most recent solves.
- A display that shows the average of your 5 most recent solves (currently includes best/worst solve, which will be changed to exclude those at some point).
- An automatic shuffle generator.

In a future version, I will probably add the ability to save your solves to a file (via the log button), then let you view that file as a graph of your solve times over time.

## Controls

- `Space` lets you start and stop the timers. Once you have completed a solve, you are able to press it again to log the solve. It does nothing during inspection.
- `Escape` discards the current solve times. If you do not want to log a particular time, this is the button you should press instead of `Space`. It can stop the inspection timer, as it resets it to 15s.
- `←` and `→` are an alternate way of controlling the number of moves in the displayed shuffle. 

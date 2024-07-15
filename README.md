# cube-timer
A simple program for speedcubing made using the Iced framework.

## Features

- Dual inspection and solve timer
- Personal best display
- Average time over the last five runs, excluding the best and worst runs
- Average over all recorded runs
- Solve saving system (this will eventually be used to implement a system to track your solve times over time)

## Controls

- `Space` lets you start and stop the timers. Once you have completed a solve, you are able to press it again to log the solve. It does nothing during inspection.
- `Escape` discards the current solve time. If you do not want to log a particular time, this is the button you should press instead of `Space`. If you decide partway through a run that you want to stop, you can also use this to reset back to inspection.
- `←` and `→` are an alternate way of controlling the number of moves in the displayed shuffle. 

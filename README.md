# cube-timer
A simple program for speedcubing made using the Iced framework.

# Features

- Separate inspection and solve timers.
- A display that shows the 5 most recent solves.
- A display that shows the average of your 5 most recent solves (currently includes best/worst solve, which will be changed to exclude those at some point).
- An automatic shuffle generator.

In a future version, I will probably add the ability to save your solves to a file (via the log button), then let you view that file as a graph of your solve times over time.

# Controls

- `Space` lets you start and stop the timers. It does nothing during inspection.
- `Escape` discards the current solve times. It can stop the inspection timer, as it resets it to 15s.
- `L` logs your solve time to the display.
- `←` decrements the number of moves in the displayed shuffle.
- `→` incremements the number of moves in the displayed shuffle.

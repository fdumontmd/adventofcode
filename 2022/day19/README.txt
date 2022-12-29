Optimizations:

from https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0ty6zr/?utm_source=share&utm_medium=web2x&context=3
Stop making ore/clay bots when we have enough to build any bot in 1 step (makes part 2 run 350x faster)

Always build geode bot if we have resources for it (makes part 2 run 30x faster)

Don't build ore/clay bots in the last few steps (makes part 2 run 2x faster)

also? https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0tz0t3/?utm_source=share&utm_medium=web2x&context=3
If one state is strictly worse than another (e.g. has the same number of robots but fewer of all resources types), discard it


use a BFS with the time as unit: at time n, we create a list of possible states for n+1
encode the time in the state, so that when we change from n to n+1, we can show the length of states

Read the instructions carefully, and do not, I repeat, do not spend 2 hours debugging the wrong part of the code

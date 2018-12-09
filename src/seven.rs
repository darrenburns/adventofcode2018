/*
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
*/

/*
 Build a map of pre-requisites and follow it e.g. map C -> [A, F] above,
 but only go one letter at a time. We're can perform a slightly modified breadth first search here.

MAP
 C -> [A, F]  # i.e. C is a pre-requisite of A and F
 A -> [B, D]
 B -> [E]
 D -> [E]
 F -> [E]

Also build a map of "blockers", i.e. the inverse of the above
 E -> [B, D, F]  # the pre-requisites of E are B, D, and F. We cannot use E (E is blocked) until these have been satisfied.
 A -> [C]
 F -> [C]
 and so on ...

QUEUE:
  E, F

For each element of the queue, we need to store it's pre-requisites. We can only pop an item
from the queue if all of its prerequisites are met.

How do we find where to start?
- Only one letter will not appear as a key in our pre-requisites map. This is the starting letter.

At which point in the algorithm do we mark of pre-requisites?
-

find the only element with no pre-requisites (C)
goto C
lookup the first map to see what C is a pre-requisite of, and find A, F
for each P of the pre-requisites A, F:
    remove C as a blocker for P

add elems to queue
sort queue
pop off first elem with all met pre-requisites (A)  # i.e. an empty "blocker" list (or all blockers marked resolved)
goto A
add elems to queue
sort queue
pop off first elem with all met pre-requisites (B)
goto B
add elems to queue
sort queue
pop off first elem with all met pre-requisites (D)
goto D
add elems to queue
sort queue

and so on...

*/



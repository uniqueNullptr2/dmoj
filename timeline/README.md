# DMPG '17 G4 - Timeline

https://dmoj.ca/problem/dmpg17g4

Harold is, like the rest of his coworkers, working hard to ensure all records are "accurate". To do this they must change some labels here and there, but more importantly, change the time that certain events occurred… in the database.

Unfortunately for you, many students are looking up "information" for their projects at the same time, and all the queries are grouped together. You wouldn't want to keep them waiting.

The timeline begins with N N events, each occurring at a distinct time t i t_i and having a label l i l_i . Initially the search filter allows all events to pass. There will be Q Q queries, which will be one of the following:

- T a b - find the event which occurred at time a and change it so it occurred at time b .
- L a b - find the event which occurred at time a and change its label to b .
- F s v - change the filter. If s is <, allow only events with labels below v to pass, if s is >, allow only events with labels above v to pass, and if s is ., reset the filter to none and ignore v.
- S v - find the event which occurred at time closest to v and passing the filter. If there is a tie, choose the later one. Output this event's time.

## Constraints

For all subtasks:

ti ≤ 10^8
li ≤ 10^9

## Subtask 1 [30%]

1 ≤ N , Q ≤ 10^3

## Subtask 2 [70%]

1 ≤ N , Q ≤ 10^5

## Input Specification

The first line will contain N.

The next N lines will each contain ti and li separated by a space.

The following line will contain Q.

The next Q lines will each contain a query in the form described above.

## Output Specification

The result of each query, one per line.

## Sample Input

```
5
-2 1
0 4
3 -1
4 3
7 4
13
S -1
F > 3
S -4
S 4
T 7 9
S 4
F < -2
L 9 -3
S -3
F . 0
S 2
T 3 -3
S -3
```
## Sample Output

```
0
0
7
0
9
3
-3
```

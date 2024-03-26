# 5 Foxholes

Playing with Rust. The current output is given below.

With 5 holes, there are 4 strategies of length 6 that guarantee a win, with the average number of steps being 71/20 = 3.55.

We also find winning strategies of length 7 with a lower average number of steps: 2.9625.

It remains to be verified if there are strategies with average number of steps strictly less than 2.9625 (for any number of steps).

```
--- brute force (5, 6) ---
Generated 72 paths of length 6 for 5 holes
  << Strategy [2, 3, 4, 2, 3, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(18)(20.000%) 2=(12)(10.000%) 3=(6)(10.000%) 4=(18)(30.000%) 5=(12)(15.000%) 6=(6)(15.000%)
  << Strategy [2, 3, 4, 4, 3, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(18)(20.000%) 2=(12)(10.000%) 3=(6)(10.000%) 4=(18)(30.000%) 5=(12)(15.000%) 6=(6)(15.000%)
  << Strategy [4, 3, 2, 2, 3, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(18)(20.000%) 2=(12)(10.000%) 3=(6)(10.000%) 4=(18)(30.000%) 5=(12)(15.000%) 6=(6)(15.000%)
  << Strategy [4, 3, 2, 4, 3, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(18)(20.000%) 2=(12)(10.000%) 3=(6)(10.000%) 4=(18)(30.000%) 5=(12)(15.000%) 6=(6)(15.000%)
We have tested 15,625 strategies (expected "15,625")
Total iterations 15,625 x 72 = 1,125,000

--- brute force (5, 7) ---
Generated 126 paths of length 7 for 5 holes
  << Strategy [1, 2, 3, 4, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.750
     Stats 1=(18)(20.000%) 2=(18)(10.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [1, 2, 3, 4, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.750
     Stats 1=(18)(20.000%) 2=(18)(10.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [1, 4, 3, 2, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.450
     Stats 1=(18)(20.000%) 2=(36)(30.000%) 3=(12)(5.000%) 4=(6)(5.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [1, 4, 3, 2, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.450
     Stats 1=(18)(20.000%) 2=(36)(30.000%) 3=(12)(5.000%) 4=(6)(5.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [2, 2, 3, 4, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.037
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(12)(7.500%) 6=(10)(6.250%) 7=(5)(6.250%)
  << Strategy [2, 2, 3, 4, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 2.962
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(15)(12.500%) 6=(8)(3.750%) 7=(4)(3.750%)
  << Strategy [2, 3, 4, 2, 3, 4, 1] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 2, 3, 4, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 2, 3, 4, 3] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 2, 3, 4, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 2, 3, 4, 5] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 4, 3, 2, 1] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 4, 3, 2, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 4, 3, 2, 3] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 4, 3, 2, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 3, 4, 4, 3, 2, 5] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [2, 4, 3, 2, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.038
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(12)(7.500%) 6=(10)(6.250%) 7=(5)(6.250%)
  << Strategy [2, 4, 3, 2, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 2.962
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(15)(12.500%) 6=(8)(3.750%) 7=(4)(3.750%)
  << Strategy [3, 2, 3, 4, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.600
     Stats 1=(36)(20.000%) 2=(18)(20.000%) 3=(12)(10.000%) 4=(6)(10.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [3, 2, 3, 4, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.600
     Stats 1=(36)(20.000%) 2=(18)(20.000%) 3=(12)(10.000%) 4=(6)(10.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [3, 4, 3, 2, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.600
     Stats 1=(36)(20.000%) 2=(18)(20.000%) 3=(12)(10.000%) 4=(6)(10.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [3, 4, 3, 2, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.600
     Stats 1=(36)(20.000%) 2=(18)(20.000%) 3=(12)(10.000%) 4=(6)(10.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [4, 2, 3, 4, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 2.962
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(15)(12.500%) 6=(8)(3.750%) 7=(4)(3.750%)
  << Strategy [4, 2, 3, 4, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.037
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(12)(7.500%) 6=(10)(6.250%) 7=(5)(6.250%)
  << Strategy [4, 3, 2, 2, 3, 4, 1] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 2, 3, 4, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 2, 3, 4, 3] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 2, 3, 4, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 2, 3, 4, 5] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 4, 3, 2, 1] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 4, 3, 2, 2] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 4, 3, 2, 3] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 4, 3, 2, 4] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 3, 2, 4, 3, 2, 5] is a winner (max 6)>>
     Average number of steps taken 3.550
     Stats 1=(27)(20.000%) 2=(18)(10.000%) 3=(9)(10.000%) 4=(36)(30.000%) 5=(24)(15.000%) 6=(12)(15.000%) 7=(0)(0.000%)
  << Strategy [4, 4, 3, 2, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 2.963
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(15)(12.500%) 6=(8)(3.750%) 7=(4)(3.750%)
  << Strategy [4, 4, 3, 2, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.038
     Stats 1=(27)(20.000%) 2=(36)(30.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(12)(7.500%) 6=(10)(6.250%) 7=(5)(6.250%)
  << Strategy [5, 2, 3, 4, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.450
     Stats 1=(18)(20.000%) 2=(36)(30.000%) 3=(12)(5.000%) 4=(6)(5.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [5, 2, 3, 4, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.450
     Stats 1=(18)(20.000%) 2=(36)(30.000%) 3=(12)(5.000%) 4=(6)(5.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [5, 4, 3, 2, 2, 3, 4] is a winner (max 7)>>
     Average number of steps taken 3.750
     Stats 1=(18)(20.000%) 2=(18)(10.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
  << Strategy [5, 4, 3, 2, 4, 3, 2] is a winner (max 7)>>
     Average number of steps taken 3.750
     Stats 1=(18)(20.000%) 2=(18)(10.000%) 3=(24)(15.000%) 4=(12)(15.000%) 5=(27)(20.000%) 6=(18)(10.000%) 7=(9)(10.000%)
We have tested 78,125 strategies (expected "78,125")
Total iterations 78,125 x 126 = 9,843,750

--- show paths (5 holes, 3 days) ---
Generated 14 paths of length 3 for 5 holes
Path: [1, 2, 1], Probability: 0.1
Path: [1, 2, 3], Probability: 0.1
Path: [2, 1, 2], Probability: 0.1
Path: [2, 3, 2], Probability: 0.05
Path: [2, 3, 4], Probability: 0.05
Path: [3, 2, 1], Probability: 0.05
Path: [3, 2, 3], Probability: 0.05
Path: [3, 4, 3], Probability: 0.05
Path: [3, 4, 5], Probability: 0.05
Path: [4, 3, 2], Probability: 0.05
Path: [4, 3, 4], Probability: 0.05
Path: [4, 5, 4], Probability: 0.1
Path: [5, 4, 3], Probability: 0.1
Path: [5, 4, 5], Probability: 0.1

--- check a strategy (3 holes, 2 days) ---
Generated 4 paths of length 2 for 3 holes
  >> Strategy [2, 2]
  >> Strategy summary for [2, 2]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 1.6667 (max 2)

--- check another strategy (5 holes, 6 days) ---
Generated 72 paths of length 6 for 5 holes
  >> Strategy [2, 3, 4, 2, 3, 4]
  >> Strategy summary for [2, 3, 4, 2, 3, 4]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 3.5500 (max 6)

--- check more strategies (5 holes, 7 days) ---
Generated 126 paths of length 7 for 5 holes
  >> Strategy [2, 3, 4, 2, 3, 4]
  >> Strategy summary for [2, 3, 4, 2, 3, 4]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 3.5500 (max 6)
  >> Strategy [4, 2, 3, 4, 2, 3, 4]
  >> Strategy summary for [4, 2, 3, 4, 2, 3, 4]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 2.9625 (max 7)
  >> Strategy [2, 2, 3, 4, 4, 3, 2]
  >> Strategy summary for [2, 2, 3, 4, 4, 3, 2]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 2.9625 (max 7)
  >> Strategy [4, 2, 3, 4, 4, 3, 2]
  >> Strategy summary for [4, 2, 3, 4, 4, 3, 2]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 3.0375 (max 7)
  >> Strategy [2, 2, 3, 4, 2, 3, 4]
  >> Strategy summary for [2, 2, 3, 4, 2, 3, 4]
  >> This is a winning strategy (0 fails)
  >> Average number of steps taken is 3.0375 (max 7)

--- check naive strategy (5 holes, 6 days) ---
Generated 72 paths of length 6 for 5 holes
  >> Strategy [3, 2, 3, 2, 3, 2]
Does not catch: [2, 1, 2, 1, 2, 1]
Does not catch: [2, 1, 2, 1, 2, 3]
Does not catch: [2, 1, 2, 3, 2, 1]
Does not catch: [2, 1, 2, 3, 2, 3]
Does not catch: [2, 1, 2, 3, 4, 3]
Does not catch: [2, 1, 2, 3, 4, 5]
Does not catch: [2, 3, 2, 1, 2, 1]
Does not catch: [2, 3, 2, 1, 2, 3]
Does not catch: [2, 3, 2, 3, 2, 1]
Does not catch: [2, 3, 2, 3, 2, 3]
Does not catch: [2, 3, 2, 3, 4, 3]
Does not catch: [2, 3, 2, 3, 4, 5]
Does not catch: [2, 3, 4, 3, 2, 1]
Does not catch: [2, 3, 4, 3, 2, 3]
Does not catch: [2, 3, 4, 3, 4, 3]
Does not catch: [2, 3, 4, 3, 4, 5]
Does not catch: [2, 3, 4, 5, 4, 3]
Does not catch: [2, 3, 4, 5, 4, 5]
Does not catch: [4, 3, 2, 1, 2, 1]
Does not catch: [4, 3, 2, 1, 2, 3]
Does not catch: [4, 3, 2, 3, 2, 1]
Does not catch: [4, 3, 2, 3, 2, 3]
Does not catch: [4, 3, 2, 3, 4, 3]
Does not catch: [4, 3, 2, 3, 4, 5]
Does not catch: [4, 3, 4, 3, 2, 1]
Does not catch: [4, 3, 4, 3, 2, 3]
Does not catch: [4, 3, 4, 3, 4, 3]
Does not catch: [4, 3, 4, 3, 4, 5]
Does not catch: [4, 3, 4, 5, 4, 3]
Does not catch: [4, 3, 4, 5, 4, 5]
Does not catch: [4, 5, 4, 3, 2, 1]
Does not catch: [4, 5, 4, 3, 2, 3]
Does not catch: [4, 5, 4, 3, 4, 3]
Does not catch: [4, 5, 4, 3, 4, 5]
Does not catch: [4, 5, 4, 5, 4, 3]
Does not catch: [4, 5, 4, 5, 4, 5]
Does not catch: [5, 4, 5, 4, 5, 4]
  >> Strategy summary for [3, 2, 3, 2, 3, 2]
  >> This is not a winning strategy (37 fails)
  >> Average number of steps taken is >= 4.3000 (max 5)

--- check naive strategy (5 holes, 10 days) ---
Generated 648 paths of length 10 for 5 holes
  >> Strategy [3, 2, 3, 2, 3, 2, 3, 2, 3, 2]
  >> Strategy summary for [3, 2, 3, 2, 3, 2, 3, 2, 3, 2]
  >> This is not a winning strategy (325 fails)
  >> Average number of steps taken is >= 5.9750 (max 9)

--- some probabilities ---
Distribution of 5 holes on day 1
Generated 5 paths of length 1 for 5 holes
Hole: 1, Probability: 0.2000
Hole: 2, Probability: 0.2000
Hole: 3, Probability: 0.2000
Hole: 4, Probability: 0.2000
Hole: 5, Probability: 0.2000
Distribution of 5 holes on day 2
Generated 8 paths of length 2 for 5 holes
Hole: 1, Probability: 0.1000
Hole: 2, Probability: 0.3000
Hole: 3, Probability: 0.2000
Hole: 4, Probability: 0.3000
Hole: 5, Probability: 0.1000
Distribution of 5 holes on day 3
Generated 14 paths of length 3 for 5 holes
Hole: 1, Probability: 0.1500
Hole: 2, Probability: 0.2000
Hole: 3, Probability: 0.3000
Hole: 4, Probability: 0.2000
Hole: 5, Probability: 0.1500
Distribution of 5 holes on day 4
Generated 24 paths of length 4 for 5 holes
Hole: 1, Probability: 0.1000
Hole: 2, Probability: 0.3000
Hole: 3, Probability: 0.2000
Hole: 4, Probability: 0.3000
Hole: 5, Probability: 0.1000
Distribution of 5 holes on day 5
Generated 42 paths of length 5 for 5 holes
Hole: 1, Probability: 0.1500
Hole: 2, Probability: 0.2000
Hole: 3, Probability: 0.3000
Hole: 4, Probability: 0.2000
Hole: 5, Probability: 0.1500

Elapsed time: 2.86s
```

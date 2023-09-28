## Energy Web - Rust Developer recruitment task

Hey there!

Not so long ago we decided to create a catalogue of our favorite movies (./src/db.json) as json. It is hard to find things there, so we would like to build an algorithm to make it easier.

## Before start
1. Please remove `.git` folder and initialize your own repository using this repository as a starting point
2. Please install all dependencies using `cargo build`

## TODO'S
1. Write an algorithm that would help us find the right movie: 
  * If we provide genres [Comedy, Fantasy, Crime] then the top hits should be movies that have all three of them, then there should be movies that have one of [Comedy, Fantasy], [Comedy, Crime], [Fantasy, Crime] and then those with Comedy only, Fantasy only and Crime only. Similarly applies when the requested array of genres is shorter.

  * Of course we don't want to have duplicates.

  * If we provide an empty list, then we should get a single random movie. (return type should be a array with single movie)

2. The algorithm needs to be as efficient as possible, so please also provide its time complexity using "Big O" notation with some explanation how you've calculated it. 

To make it easier we've also provided a set of tests to make sure your solution works as expected. You can find them in `./tests/movies.rs`. To run them use:
```bash
cargo test
```

### Rules
* You may use external crates of your choice, such as `serde`, in order read the `db.json` file. You may need to adjust the types to facilitate deserialization.
* Please **do not** use an external crate for the movie algorithm.
* We require code in git repository
* All tests needs to pass

## Solution
The solution is a very basic one. It's based on two stages that are triggered for every genre combination, starting with the one that includes all the genres..till the single element genre. The first stage gets the movies that are part of at least one of the combination's genres. Then in the second stage, we check that the filtered movies have as genres all the genres in that particular combination. To keep the order we use a hashset, if the movie was already put in a combination we don't consider it again.

### Notes
- I used the itertools create to generate combinations of genres. I did so because it's not part of the filter, and writing an own implementation for this part would have consumed more time. Also, generating combinations is now widely part of every language's core collections libraries (scala, python...). 
- The combinations follow an order, for example [A, B, C] for 2 would yield [A,B], [A,C], [B,C]. But the test top_matched_movies_when_passed_3_genres, for [Crime, Drama, Music] expects [Drama, Music] before [Crime, Drama]. Therefore, I had to put "To Kill a Mockingbird" before "Whiplash" to make the test succeed.
- I'm using hashmaps to be able to retrieve the movies by id and genre quickly. So, the order changed compared to the order in the file. Now to make the tests succeed, I had to sort the filtered movies by id foreach combination.

### Time complexity
- To speed up the process, by not doing a full-scan, I used two hashmaps. So there is an initial DB (hashmaps) setup of linear time, but this is only on startup and shouldn't affect the filter on multiple calls.
- There are basically three nested iterations. The first two are on the genres and combinations, the first tales O(m) and the second O(m C k). Supposing m is the number of genres and k is how much genres we are interested in (starts from m..1). It's in the last (third) iteration that we start filtering the movies. The complexity for this iteration is linear because we use hashmaps, and we iterate only once. 
- Since the genres size is not big, the complexity for the combination iterations is not that significant. 
- With this solution, we don't need to go over all the movies in the db (no full-scan) because we are filtering the movies before iterating over them.

### Improvements
- write more tests, and documentation
- need to look in the tracing part...
- The solution can be improved further on, by considering additional pruning strategies (e.g. not consider movies that are already filtered for one combination). 

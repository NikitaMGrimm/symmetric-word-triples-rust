## TODOs
- Diagonal optimization? (If you have a solution, you can vary the diagonal chunks to get more solutions)

- Trie instead of hashmap and fst set!!! Two in one improvement. Then you can also make the entire program only accept chunky words. No conversion needed!!!
  - Will be tricky to implement with diagonal optimization. Might need to make another fst set for the rare regex searches with wildcards.
  
- Dashset for solution_set_file (or similar)
  - Instantly add into hashmap if you have a solution instead of collecting and appending.
  - Also: Somehow dump the results into a file during the computation (instead of waiting for the whole thing to finish) (high RAM usage)

- In the hot part, iterate over a finished chunky word dict instead of calling chunkify for each word.
  - Pretty much just pre-chunkify everything for each file beforehand and only accept chunky words everywhere.

- Make new matrix struct for chunky to avoid indirection, make methods to work on flat vector with offsets

- Install cargo bloat

- Get should return customoption<option, maybe new enum for option that can be unwrapped to differentiate between not found and empty set
  - Might not be necessary because empty vectors are not allocated

Diagonal optimization:
0.
split words into:  prefix + suffix
    abr -> r + ab
    mgr -> m + gr
    agr -> a + gr
Do this for each partition of prefix + suffix possible. 
Make a hashmap with prefix as key and suffix as value.
But also make a hashmap with suffix as key and prefix as value.

1.1
? a b c -> suffix of length 3. loop over all of them. (Possible solutions for first row)
_ ? _ _
_ _ ? _
_ _ _ ?

1.2
? a b c
a ? _ _ -> get all words that begin with a. (= "these words") (we already have those). Get the set of all suffixes of "these words" with suffix length 2. loop over 
b _ ? _    all of them. (Possible solutions for second row) hash these every time. (remember the prefixes of "these words" as they solve the ?)
c _ _ ?

2.1
? a b c
a ? d e 
b _ ? _
c _ _ ?

2.2
? a b c
a ? d e
b d ? _ -> get all words that begin with bd. (= "these words") (we already have those). Get the set of all suffixes of "these words" with suffix length 1. loop over
c e _ ?    all of them. (Possible solutions for third row) hash these every time (remember the prefixes of "these words" as they solve the ?)

3.1
? a b c
a ? d e 
b d ? f
c e _ ?

3.2
? a b c
a ? d e 
b d ? f
c e f ? -> last row -> return the possible suffixes of cef (which we have) because they solve the ?

END:
We have gotten the map from 3.2 and we have the solutions for all the ? in every row. Iterate over all of them. Add them to the solution set.
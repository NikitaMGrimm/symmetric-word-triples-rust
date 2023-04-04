## TODOs
- Will be tricky to implement with diagonal optimization. Might need to make another fst set for the rare regex searches with wildcards.
- Make everything use slices instead of vectors. 

- Diagonal optimization? (If you have a solution, you can vary the diagonal chunks to get more solutions) VERY HARD!
  
- Dashset for solution_set_file (or similar)
  - Instantly add into hashmap if you have a solution instead of collecting and appending.
  - Also: Somehow dump the results into a file during the computation (instead of waiting for the whole thing to finish) (high RAM usage)
  - Somehow save your state in the middle of the computation (so you can continue later on)

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

Scuffed approach:
Do the thing I now do but every time I get the next prefix words, replace the diagonal chunk with a placeholder like ??? and put all the chunks that were
replaced into a set to get all solutions later on. While you replace the chunk with ???, put them into a hashset to quickly remove duplicates.
Make it a hashmap from 

Then iterate over this and do the usual, get the next prefix, etc...

Then after you have found a solution, iterate over all the ??? and replace them with the possible chunks that we remembered earlier.
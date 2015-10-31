==Similar Sentences==

build with ```cargo build --release``` to take advantage of optimizations.

use with ```zcat sentences.zip | lsh``` to avoid unzipping a 500 MB file.

The binary reads sentences of structure ID word1 word2 ... wordN and calculates pairs 
that have a maximum edit distance of 1 (one).


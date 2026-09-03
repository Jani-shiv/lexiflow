# AI & Grammar Model Architecture

## Model Engineering & Memory Budget (<100MB RAM)
To achieve sub-100MB resident process memory and instant (<5ms) inference without GPU or cloud APIs, Personal Grammar Enhancer combines:

1. **Deterministic Rule Engine**:
   - Covers high-precision grammar patterns: Subject-Verb Agreement, Indefinite Article (`a`/`an`) Agreement, Tense & Modals (`could of` -> `could have`), Phrasing & Missing Prepositions (`I am go office` -> `I am going to the office`), Homophones (`their/there/they're`), and Capitalization.
   - Memory footprint: ~2.5 MB.
   - Execution latency: ~20 µs.

2. **Spell Checking Dictionary & Trie**:
   - Compact vocabulary table with common misspellings database and Levenshtein edit distance.
   - Memory footprint: ~3.0 MB.
   - Execution latency: ~40 µs.

3. **Statistical N-Gram Language Scorer**:
   - Evaluates bigram/unigram collocation probability with Laplace smoothing to rank candidate fluency.
   - Memory footprint: ~4.5 MB.
   - Execution latency: ~30 µs.

### Total Resident Memory Footprint
- Peak normal runtime memory: **~35 - 55 MB** (well below the 100 MB hard limit).
- Average inference time: **< 1.0 ms**.

---
name: aoc-day
description: Solve an Advent of Code 2024 day in this Rust repo end-to-end. Use when the user asks to "solve day N", "do AoC day N", implement a day's puzzle, or add/run a day's part one/two. Fetches the puzzle text from adventofcode.com (using session_secret), scaffolds the dayXX.rs file and registration, builds the test input, proposes and implements a solution, verifies against the example, runs the real input, and submits the answer to the site (submitting part one unlocks part two automatically).
---

# Advent of Code 2024 — solve a day

End-to-end workflow for implementing one day's puzzle in this repo. Work **one part at a time**: get part one fully correct (test then real), **submit it** (step 6) — which unlocks part two on the site — then do part two and submit it. Submitting from the skill means there's no need to wait for the user to unlock part two manually.

## 0. Inputs you need

- **Day number `X`** (1–25). If the user didn't say, ask or pick the lowest day whose `get_solver` arm in `src/base/day_marker.rs` is still `todo!()`.
- The repo root has a `session_secret` file (gitignored) holding the AoC session cookie. It is required to fetch the input and part two. Read it trimmed: `$(cat session_secret | tr -d '\n')`.

Throughout, `X` is the plain number (CLI/enum) and `XX` is **zero-padded** to 2 digits (file/struct names): day 6 → file `day06.rs`, struct `Day06`, but `--day 6`.

## 1. Fetch the puzzle description

Part one is public; part two only appears once part one is solved (needs the cookie). Fetch the rendered text:

```bash
curl -s -H "Cookie: session=$(cat session_secret | tr -d '\n')" \
  -A "github.com/aoc-helper by user" \
  https://adventofcode.com/2024/day/X \
  | sed 's/<[^>]*>//g' | grep -v '^\s*$'
```

Read it carefully and pull out:
- The **rules / what to compute** for the current part.
- The **example input** — usually the first `<pre><code>` block. Preserve it **character for character**; transcription typos cause wrong answers (verify by re-fetching, don't hand-retype large grids).
- The **expected example answer(s)** — the bold numbers in the prose (e.g. "the result is **7036**"). Part two often reuses the same example with a different expected answer; sometimes it gives a new example.

`WebFetch` does NOT work for authenticated content (no part two) — always use the `curl` form above.

## 2. Create the test input

Write the example grid/data verbatim to:

```
inputs/test_input_X.txt
```

(plain `X`, not zero-padded — matches `get_input`). Real inputs live at `inputs/input_X.txt` and download automatically (step 5).

## 3. Scaffold the day file (if missing)

If `src/puzzles/dayXX.rs` does not exist, create it:

```rust
use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct DayXX;

impl DaySolver<u64> for DayXX {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        Ok(0)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        Ok(0)
    }
}
```

`input.input` is the whole file as a `String`. Useful shared helpers:
- `crate::utils::matrix::{Matrix, Direction, NodeInfo}` — 2D grid with `get`, `set`, `get_next_info`, `find_first`, `find_all`, `count_all`, neighbour helpers, 8 `Direction`s. Parse a grid with `Matrix::new(rows, cols, data)`.

## 4. Register the day

Only needed the first time the day is created. The `DayMarker` enum, `from_day_number`, and `get_input` already list all 25 days — do **not** touch those. Make exactly these edits:

1. `src/puzzles/mod.rs` — add `pub mod dayXX;` (keep order).
2. `src/base/day_marker.rs` — add `use crate::puzzles::dayXX::DayXX;` with the other imports.
3. `src/base/day_marker.rs` — in `get_solver`, replace `DayMarker::DayXX => todo!("Not implemented yet!"),` with `DayMarker::DayXX => Box::new(DayXX),`.

`main.rs` needs no changes — the day is selected via `--day X`.

## 5. Implement, then verify on the test input first

1. **Propose the approach** to the user briefly (algorithm + data structures) before writing non-trivial code.
2. Implement `solve_first` (or `solve_second`).
3. Build and run on the **test** input:

   ```bash
   cargo run -- --day X --test
   ```

   The output logs `First part: <n>` and `Second part: <n>`. Compare against the expected example answer from step 1.
4. **If it doesn't match**, debug (re-check the example transcription, the parse, edge cases) and iterate. Do not proceed until the example matches.
5. **Once the example matches**, run on the real input (auto-downloads via `session_secret` on first run):

   ```bash
   cargo run -- --day X
   ```

   Report the real answer, then submit it (step 6).

## 6. Submit the answer

After the real run, submit the answer with a POST to the `/answer` endpoint. Submitting **part one** flips the site to part two — no manual unlock needed. `level` is `1` for part one, `2` for part two.

```bash
curl -s -X POST \
  -H "Cookie: session=$(cat session_secret | tr -d '\n')" \
  -A "github.com/aoc-helper by user" \
  --data-urlencode "level=1" \
  --data-urlencode "answer=<ANSWER>" \
  https://adventofcode.com/2024/day/X/answer \
  | sed 's/<[^>]*>//g' | grep -v '^\s*$'
```

- Use `--data-urlencode` so answers with commas/special chars (e.g. day 17's `5,0,3,5,7,6,1,5,4`) are encoded correctly.
- **Submit the actual puzzle answer string the user would type**, which may differ from the framework's `u64` return value (e.g. when a part's answer is a comma-joined string and the solver returns a numeric stand-in — submit the string, not the stand-in).
- Read the response prose:
  - **"That's the right answer"** → accepted. For part one, part two is now unlocked.
  - **"That's not the right answer"** (sometimes "too high"/"too low") → wrong. **Do not blindly resubmit**; re-examine the solution. Wrong guesses trigger a cooldown.
  - **"You gave an answer too recently"** → rate-limited; wait the stated time before retrying.
  - **"You don't seem to be solving the right level. Did you already complete it?"** → that level is already done (e.g. a duplicate submit); move on.

Then move to part two: re-fetch the page (step 1) to read the now-unlocked part two, set up its example expectation, implement `solve_second`, repeat the test-then-real verification (step 5), and submit with `level=2` (step 6).

## Gotchas / notes specific to this repo

- The input downloader reads `session_secret` and trims it; if a download fails with a header-parse error, confirm `session_secret` has no stray characters.
- `Direction` derives `Hash, Ord, PartialOrd` (besides `Eq`), so it can be a `HashMap`/`BinaryHeap` key directly.
- Solutions return `Result<u64, anyhow::Error>`; timing per part is logged automatically.
- Keep part-one code lean if part-two reuses it; only share helpers when it doesn't slow part one.
- Verify against **every** example answer the description gives (part one and part two examples can differ) before trusting the real run.

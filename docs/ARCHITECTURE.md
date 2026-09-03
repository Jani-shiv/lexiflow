# Architecture Documentation

## Overview
**Personal Grammar Enhancer** is structured into distinct, modular subsystems to ensure safety, speed, maintainability, and strict adherence to the **< 100 MB memory limit**.

## Subsystem Breakdown

### 1. Platform & Text Acquisition (`src/platform/`)
- Interacts with operating system accessibility mechanisms (Windows UI Automation, Win32 hooks) to observe text context non-intrusively.
- Obtains only the active sentence rather than global key logging history.
- Distinguishes between native input and internal injected text using `InjectionGuard`.

### 2. Context Buffer & Sentence Boundary Segmenter (`src/text_context/`, `src/sentence_detection/`)
- `TextContextBuffer`: Bounded, short-lived ring buffer with automatic TTL eviction. Never writes to disk.
- `SentenceSegmenter`: Disambiguates punctuation periods from abbreviations (`Mr.`, `Dr.`, `U.S.`, `i.e.`), decimal numbers (`3.14`), URLs, and emails.

### 3. Debounce Scheduler & Versioning (`src/scheduler/`)
- Implements non-blocking, asynchronous queuing between input threads and AI inference workers.
- Each keystroke generates a monotonically incrementing atomic `request_id`.
- If a newer keystroke occurs while background inference is in progress, earlier inference results are instantly marked stale and dropped.

### 4. Hybrid Grammar Engine (`src/grammar/`)
- **Deterministic Rules (`rules.rs`)**: High-speed regex and token matching for subject-verb agreement, common typos, tense mistakes, preposition omissions, and capitalization.
- **Spell Dictionary (`dictionary.rs`)**: Compressed dictionary with typo lookup and Damerau-Levenshtein edit-distance search.
- **Statistical Language Model (`statistical.rs`)**: Compact n-gram frequency table scoring transition fluency in under 100 microseconds.

### 5. Confidence Filter & Diff Generator (`src/confidence/`, `src/diff/`)
- Discards low-confidence candidate fixes (< 0.85).
- Calculates the minimal common prefix/suffix diff slice to replace only the necessary characters.

### 6. Text Injector & Clipboard Protection (`src/replacement/`)
- Verifies application identity, cursor context, and request version prior to injection.
- Preserves user clipboard format and data during replacement simulations.

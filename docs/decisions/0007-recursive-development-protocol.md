# ADR 0007: Recursive Development Protocol

## Status

Accepted.

## Decision

Fate-Track will use a recursive development loop for code development, refactoring, and milestone execution after the user explicitly confirms that implementation may begin.

The recursive loop is defined in:

- `markdown/20-roadmap/95-recursive-development-protocol.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`

The loop cursor is an active governance artifact. It records the current milestone, work package, state, allowed scope, forbidden scope, decision gates, locks, validation commands, last closeout, and next resume instruction.

## Recursive Contract

Each loop must follow:

```text
Read -> Slice -> Preflight -> Implement -> Govern -> Validate -> Closeout -> Advance -> Repeat
```

Before implementation begins, the cursor remains in `design_only` mode. In `design_only`, only process and governance documents may change; business code, API behavior, frontend functionality, and capability promotion are forbidden.

## Rationale

The project has a large governance surface: milestone files, ADRs, research reports, module tree, engineering tree, risk register, capability ledger, and quality gates. A recursive loop prevents progress from becoming untraceable by making each development pass return a structured closeout result and update a cursor.

This matches the user's QuantPilot-derived governance experience: development should proceed as repeated closed loops, not as an unbounded implementation stream.

## Consequences

- Code work must name the active loop and cursor state before implementation.
- Each loop must write or update closeout evidence.
- Capability status cannot change unless the loop records the evidence and updates the capability ledger.
- If the loop is blocked by a decision gate, S0 risk, or failed validation, the cursor must move to `blocked` or `paused`, not silently continue.
- Goal-style long runs may only begin after user approval and after the protocol has proven stable through successful smaller loops.


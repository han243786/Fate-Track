# M51 - Main Report Tone Cohesion

## 1. Goal

M51 is a quality-only polish loop for the main chart report. It uses the current relationship report as the strongest style reference, while preserving M47-M50 gates for all topic reports.

The target is to move the main report from a teaching-manual tone toward a direct reader-facing chart reading.

## 2. Scope

In scope:

- Rewrite main chart chapter openings so they no longer begin with repeated `这一章看的是` / `这一章先把`.
- Reduce visible teaching bridges such as `放到日常理解里`, `最适合当作`, and `可以先这样理解`.
- Replace main ten-god count summaries such as `一处` / `两处` style inventory with qualitative ten-god signal language.
- Keep explicit-year annual and major-luck readings restricted, scoreless, and non-deterministic.
- Regenerate real samples and scan all five reports before closeout.

Out of scope:

- No route change.
- No DTO top-level shape change.
- No `/api/capabilities` change.
- No supported promotion.
- No raw `GET /api/luck/cycles` reading pollution.
- No public `score_internal` or 0-100 score.
- No flow-month, flow-day, event schedule, daily fortune, financial outcome, family event, career result, partner identity, or deterministic real-world claim.

## 3. Guardrails

M51 must preserve:

- M47 relationship six-block golden sample.
- M48 wealth/family/career count-field narrative baseline.
- M49 annual/timeline narrative baseline.
- M50 wealth/family/career advice-cohesion baseline.

M51 adds a main-report public-copy guard against:

- `这一章看的是`
- `这一章先把`
- `放到日常理解里`
- `最适合当作`
- `可以先这样理解`
- ten-god inventory wording that exposes `一处` / `两处` as the main summary style

## 4. Closeout Evidence

Required before closeout:

- `cargo test report -- --nocapture`
- `cargo test topic_report -- --nocapture`
- `cargo test relationship -- --nocapture`
- Regenerated `target/report-polish-samples/main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt`
- JSON audits for all five generated samples
- M51 forbidden phrase scan
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`
- `git diff --check`

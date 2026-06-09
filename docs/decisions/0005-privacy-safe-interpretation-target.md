# ADR 0005: Privacy and Safe Interpretation Policy

## Status

Target. Policy accepted; feature implementation remains planned.

## Decision

Birth data, location data, timezone resolution, chart snapshots, derived analysis, share tokens, and private notes are governed as sensitive or private data by default.

Fate-Track interpretation must be structured, bounded, and non-deterministic. It may describe traditional Bazi structure, but it must not output deterministic health, legal, financial, relationship, fertility, death, criminality, or coercive claims.

## Data Classification

| Data | Classification | Default handling |
| --- | --- | --- |
| Birth date and time | sensitive personal | no raw logs, redacted by default in shares |
| Birth location and coordinates | sensitive personal | no raw logs, redacted by default in shares |
| Timezone ID and resolved offset | sensitive derived | stored for reproducibility |
| Bazi chart snapshot | sensitive derived | immutable and versioned when persisted |
| Analysis summary | sensitive derived / share-redacted | share only through explicit preset |
| Private notes | private | never included in public share by default |
| Share token | sensitive | store hash only when persistence exists |
| Glossary text | public | safe for anonymous read |

## Logging Prohibitions

The application must not log raw:

- birth timestamps;
- birth locations or coordinates;
- chart creation request bodies;
- complete chart JSON;
- private notes;
- access tokens, session IDs, or share tokens;
- database credentials, encryption keys, or secrets.

## Sharing Rules

Public share surfaces must default to:

- alias or empty name;
- no exact birth minute;
- no precise location or coordinates;
- no private notes;
- immutable redacted snapshot rather than live private case state;
- high-entropy unlisted token;
- revoked/expired behavior that does not reveal private case existence.

## Safe Interpretation Rules

Allowed language should be structured and bounded:

- "This chart can be read as emphasizing..."
- "This period may feel more changeable under the selected rule profile."
- "Hour-dependent conclusions are provisional because the birth hour is unknown."

Forbidden output classes include:

- diagnosis, disease certainty, death timing, fertility certainty;
- legal or financial instructions;
- guaranteed wealth, loss, marriage, divorce, or relationship events;
- certainty claims about criminality, abuse, infidelity, or moral character;
- coercive prescriptions framed as destiny.

Every generated analysis surface must include a concise disclaimer that the content is traditional, interpretive, and not professional advice.

## Consequences

- API contracts for charts and shares must carry privacy classification through DTO review.
- Frontend copy must distinguish supported calculations from interpretive or educational content.
- Tests for share redaction and forbidden claim patterns are required before sharing or generated analysis is marked supported.


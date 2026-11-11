# Fixtures

- `signed-*.json`: real signed envelopes from a BlocRA backend.
- `signed-*.md`: the Markdown file each envelope renders to.
- `render/<case>.json` + `render/<case>.md`: a report (and attestation, or null) and the exact
  body it renders to, covering partial and full periods, missing and null fields, unknown chains,
  escaping and number formatting edge cases.

The `.md` files are the contract between the two renderers: this CLI (`tests/parity.rs`) and the
BlocRA website (`blocra-frontend`, `npm run check:blocra`) must both reproduce them byte for byte,
so a report exported from either one verifies with the other. Change the report layout in both
renderers together and regenerate these files in the same change.

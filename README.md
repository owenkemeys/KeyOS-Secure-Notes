Written by Codex:

# Secure Notes Prototype

Secure Notes is a Passport Prime prototype for storing searchable private records such as secure notes, identity documents, payment cards, account details, insurance information, serial numbers, and other sensitive notes.

The app is currently a working prototype built with the Foundation Passport Prime SDK and Slint. It has been tested in the Foundation simulator, but it is not production-reviewed security software.

## Current Features

- Searchable note list with folder, type, tag, and sort filters.
- Bitwarden-oriented data model for supported item types:
  - Login
  - Secure Note
  - Payment Card
  - Identity
  - SSH Key
- Manual create/edit flows with folders, tags, favorites, custom fields, archive/restore, and delete confirmation.
- Custom fields support text, hidden text, and boolean fields.
- Per-record "Require PIN to view" setting using the KeyOS PIN verification flow.
- Bitwarden JSON import flow for unencrypted exports.
- Import review screen with fuzzy duplicate detection so users can skip similar records or import them anyway.
- Import-complete summary showing imported, skipped, and failed items.
- Design-system icon provenance is recorded in `secure-notes/resources/icons/icon-sources.json`.

## Repository Layout

```text
secure-notes/       Passport Prime GUI app and Slint UI
secure-notes-core/  Bitwarden-compatible data model and import logic
test-imports/       Sample unencrypted Bitwarden JSON exports for simulator testing
tools/              Local helper scripts for writing test files into simulator storage
*.ps1, *.sh         Windows/VM helper scripts used during prototype development
```

## Build Notes

This prototype was developed from Windows by copying source into an Ubuntu VM with the Foundation SDK installed.

The app `Cargo.toml` currently uses absolute SDK paths inside the VM:

```toml
/home/foundation/.foundation/sdk/current
```

That is expected for this prototype setup. Developers taking this over should either build in a matching VM path or update the SDK path dependencies for their environment.

The latest simulator build completed successfully with:

```text
foundation build
```

Known build warnings are unused Slint components left behind from earlier UI iterations. They are not runtime blockers, but should be cleaned up by maintainers.

## Security And Production Notes

This prototype should be treated as UX and interoperability work, not as a completed security implementation.

Important open items for production review:

- Verify KeyOS storage encryption guarantees for app data at rest.
- Decide whether app-level encryption is required in addition to KeyOS storage and sandboxing.
- Replace the prototype signing identity and publisher metadata in `secure-notes/app-config.toml`.
- Review file import handling for malformed, very large, or adversarial JSON.
- Add an export path only after deciding whether exported files should match Bitwarden's unencrypted JSON format, an encrypted format, or both.
- Add automated tests around import, duplicate detection, data migration, and failure recovery.

## Bitwarden Compatibility

The core crate mirrors Bitwarden's item type identifiers for the supported item types and stores fields in Bitwarden-shaped structures where practical. Import currently supports unencrypted Bitwarden JSON exports only. Encrypted Bitwarden exports are intentionally rejected.

The app does not yet implement export back to Bitwarden JSON. The internal model is close enough that export should be feasible, but it should be implemented and tested explicitly before claiming round-trip compatibility.

## Test Files

Sample import files are in `test-imports/`, including:

- `secure-notes-sim-seed.json`
- `bitwarden-stress-mixed-1.json`
- `bitwarden-stress-mixed-2.json`
- `bitwarden-stress-20.json`

These intentionally include a mix of new records, fuzzy matches, custom fields, hidden fields, boolean fields, and PIN re-prompt-style data.

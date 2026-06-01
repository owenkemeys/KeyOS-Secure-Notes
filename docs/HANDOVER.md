Written by Codex:

# Secure Notes Developer Handover

## Purpose

Secure Notes is intended to give Passport Prime users a small-device vault for high-value personal records that are not necessarily passwords: identity numbers, insurance details, financial account metadata, serial numbers, recovery notes, and similar sensitive text.

The prototype is already useful for UX review and simulator testing. It is not yet ready for production without a security and persistence review.

## What Works In The Prototype

- Home list with search, filters, sort order, archive access, and per-row locked-record indicator.
- Details view that does not expose private data from the home list.
- Manual create/edit flows for all supported Bitwarden item types.
- Folder assignment, tag entry, favorite flag, and per-record PIN requirement.
- Custom fields:
  - text
  - hidden text, displayed as dots on the Details screen
  - boolean, rendered as toggles
- New-folder creation from the edit screen.
- Archive, restore, and delete flows.
- Bitwarden JSON import:
  - rejects encrypted exports
  - parses supported Bitwarden item types
  - previews all importable records
  - fuzzy-matches similar existing records
  - defaults fuzzy matches to skipped
  - lets user choose which records to import
  - shows imported/skipped/failed summary afterward

## Recent UX Decisions

- Do not show secret details on the home list.
- Use one long scrollable list rather than paged navigation.
- Import lives under the ellipsis menu, not the primary New Item flow.
- Details screen reveals record details after PIN verification when a record requires PIN.
- The PIN verification header uses a fixed short label, "Restricted Note", to avoid layout collisions.
- Light mode uses the KeyOS/Figma background pattern from the Vault examples.
- Small icons use the selected Design System icon assets; provenance is in `secure-notes/resources/icons/icon-sources.json`.

## Current Architecture

`secure-notes-core` contains the data model and import logic:

- Bitwarden item types and nested structs.
- `SecureNote`, which is the app's internal record type.
- `NotesVault`, which owns the list and import/deduplication operations.
- Bitwarden preview/review/import helpers.

`secure-notes` contains the Passport Prime app:

- Slint UI in `ui/app.slint`.
- Runtime callbacks and storage wiring in `src/main.rs`.
- Theme helper in `src/theme.rs`.
- App resources in `resources/`.

The app currently uses the Foundation SDK file API and stores app data locally. Production developers should verify exact KeyOS persistence, encryption, migration, and failure behavior before shipping.

## Known Prototype Limitations

- Storage security has not been independently audited.
- No app-level encryption has been implemented.
- No export feature exists yet.
- No automated Rust test suite has been added.
- Some UI code contains unused older Slint components and should be cleaned up.
- Current app config includes prototype signing and publisher metadata.
- Build helper scripts assume the local Windows/Ubuntu VM workflow used during prototyping.

## Recommended Next Engineering Steps

1. Confirm KeyOS storage guarantees for app data and decide whether app-level encryption is required.
2. Add automated unit tests for:
   - Bitwarden item parsing
   - fuzzy duplicate matching
   - hidden/boolean custom fields
   - import selected/skipped records
   - data migration and corrupted-data recovery
3. Add an export implementation if Bitwarden round-trip compatibility is required.
4. Replace prototype signing identity and publisher metadata.
5. Remove unused Slint components and archive-only helper artifacts.
6. Review accessibility and keyboard behavior on hardware once hardware app install support is available.
7. Decide whether folder management needs a dedicated management screen or whether inline creation is enough.

## Simulator Workflow Notes

The prototype was developed using Windows plus an Ubuntu VirtualBox VM. The local helper scripts are included because they encode several hard-won workflow fixes:

- copy source without copying Rust `target/`
- pre-build cleanup only before build
- keep Weston large/maximized for handoff
- copy Bitwarden test files into the simulator's internal FAT storage under `user/`

These scripts are useful for reproducing the prototype environment but should not be treated as production build infrastructure.

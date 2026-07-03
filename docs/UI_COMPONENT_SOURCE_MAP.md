# Secure Notes UI Component Source Map

This document records the starting point for replacing Secure Notes' hand-built UI controls with official public KeyOS UI components.

## Scope

- App repo branch: `codex/ui-foundation-components`
- Backup tag before refactor work: `prototype-before-ui-component-refactor`
- Current prototype baseline commit: `8cf4c06 Prepare Secure Notes prototype handover`
- First implementation checkpoint: `716279f Adopt public KeyOS background and UI tokens`
- Public KeyOS source repo: `Foundation-Devices/KeyOS`
- Public KeyOS commit pinned for this audit: `9056b4805315cad3a8dd58f7c7d06a08e27a1a31`

The audit intentionally uses public Foundation sources only, so Secure Notes remains buildable and maintainable from an outside developer setup.

## Licensing And Attribution

Public KeyOS UI files are marked `GPL-3.0-or-later`. Any direct source reuse must preserve SPDX headers and source attribution. Prefer importing/wrapping SDK-provided widgets where available. If a component must be copied into this repo because the SDK package does not expose it, record:

- upstream file path
- upstream commit
- local file path
- whether the local file is copied unchanged or adapted
- reason for any adaptation

Do not add a new `@ui/...` import only because the public KeyOS repo has that widget. First verify the installed public SDK exposes the same import path and API. If it does not, either keep the local Secure Notes wrapper for that stage or vendor the upstream file deliberately with SPDX headers intact.

## Implemented Checkpoints

### `716279f Adopt public KeyOS background and UI tokens`

- Added official public KeyOS light and dark background PNGs from the pinned commit.
- Added `secure-notes/resources/background-sources.json` to track upstream paths and URLs.
- Replaced the previous local SVG background reference with the official PNG pair.
- Updated the Secure Notes primary/accent color from the old local teal to public KeyOS `blue-500` (`#009db9`) and pressed `blue-700` (`#006f83`).
- Updated the destructive fallback to public KeyOS `red-500` (`#ff3333`) and hardcoded notice/copper uses to `light-copper-500` (`#d68b6e`).
- Adjusted local toggle dimensions to match the public `Switch` geometry.
- Adjusted local ellipsis menu row dimensions toward the public `PopupMenuItem` geometry without importing unverified SDK widgets.

### `codex/ui-foundation-components` SDK-backed UI adapter pass

- Added `docs/KEYOS_DESIGN_SYSTEM_MAP.md` after inspecting the Design System KeyOS Figma file and the installed SDK `@ui` component APIs.
- Added local SDK snapshot notes under `Codex_Working/sdk_ui_snapshot/` for agent inspection; do not treat that snapshot as production source.
- Wrapped Secure Notes action buttons with SDK `@ui/button.slint::Button`.
- Wrapped icon-only controls with SDK `@ui/icon_button.slint::IconButton`.
- Wrapped single-line inputs with SDK `@ui/input.slint::Input`.
- Wrapped search with SDK `@ui/search.slint::Search`.
- Wrapped toggles with SDK `@ui/switch.slint::Switch` while preserving row-level tap behavior for list rows.
- Replaced the filter sheet's custom picker overlay with SDK `@ui/dropdown.slint::Dropdown` controls.
- Replaced the ellipsis menu container/rows with SDK `@ui/menu.slint::Menu` and `@ui/menu_item.slint::MenuItem`.
- Corrected the ellipsis menu structure so `MenuItem` rows are direct children of SDK `Menu`; an extra nested layout causes clipped row text and misplaced trailing icons.
- Replaced the delete confirmation popup with SDK `@ui/dialog.slint::Dialog`.
- Removed obsolete hand-drawn filter, lock, document, divider, and star components from the active UI file.

Remaining intentional local wrappers:

- `MultiLineInput`, because the current SDK `TextArea` does not expose the same Done-key dismissal behavior needed for Prime keyboard testing.
- Neutral Secure Notes list rows, because the product intentionally diverges from Vault's colorful seed-card rows.
- Import review/result rows, because their fuzzy-match and import-state layout is product-specific, although they now use official switch/icon/button primitives where practical.
- Scrollable list/detail/form regions currently use plain clipped scroll frames. A partial content-width port of production `KeyOS/ui/ui/widgets/shield.slint` was rejected because Shield is a full shell/backboard treatment, not a generic local fade strip. The rejected asset provenance is recorded in `secure-notes/resources/fader-sources.json`.
- `FieldEditButton` intentionally renders the designer SVG directly in a fixed image slot instead of using SDK `IconButton`; the current simulator rasterizes that 24px pencil more cleanly without the extra scaling layer.

## Primary Upstream Sources

| Purpose | Public KeyOS source |
| --- | --- |
| Theme tokens, palette, button colors | `ui/ui/theme.slint` |
| Background, icon, image callbacks | `ui/ui/images.slint` |
| Light and dark app background assets | `ui/ui/images/background.png`, `ui/ui/images/background-dark.png` |
| Button | `ui/ui/widgets/button.slint` |
| Icon button | `ui/ui/widgets/icon-button.slint` |
| Icon loading and sizing | `ui/ui/widgets/icon.slint` |
| Text input, focus border, errors, clear button, password reveal | `ui/ui/widgets/input.slint` |
| Label + input grouping | `ui/ui/widgets/titled-input.slint` |
| Search field | `ui/ui/widgets/search-input.slint` |
| Dropdown line and selection overlay | `ui/ui/widgets/dropdown.slint` |
| Switch/toggle | `ui/ui/widgets/switch.slint` |
| Popup menu rows and icon alignment | `ui/ui/widgets/popup-menu.slint` |
| Modal, confirmation modal, result modal, overlay menu | `ui/ui/widgets/base-page.slint` |
| Bottom drawer/sheet | `ui/ui/widgets/drawer.slint` |
| Header and back/right actions | `ui/ui/widgets/app-header.slint` |
| Page body, scroll body, content and button sections | `ui/ui/widgets/base-page.slint` |
| Panel/shadow/card chrome | `ui/ui/widgets/panel.slint` |
| Folder-style popup list | `ui/ui/widgets/folder-list.slint` |
| Vault details/edit workflow reference | `apps/gui-app-seed-vault/ui/pages/details/page.slint` |
| Vault new/import hero workflow reference | `apps/gui-app-seed-vault/ui/pages/main/page.slint` |
| File browser text-entry modal reference | `apps/gui-app-file-browser/ui/modal/widgets.slint` and `create-folder.slint` |
| Component usage examples | `ui/slintbook/ui/pages/inputs-page.slint`, `toggles-page.slint`, `nav-sort-search-page.slint` |

Stable source links should use:

`https://github.com/Foundation-Devices/KeyOS/blob/9056b4805315cad3a8dd58f7c7d06a08e27a1a31/<path>`

## Current Secure Notes UI Inventory

The main UI file is `secure-notes/ui/app.slint`. It still defines app-specific wrappers locally, but the most common controls are now SDK-backed adapters:

- SDK-backed adapters: `GlyphButton`, `ActionButton`, `FieldEditButton`, `PillInput`, `SearchInput`, `FilterButton`, `InlineToggle`, `MenuItem`, `MenuOverlay`, `FilterSheetOverlay`, `DeleteOverlay`.
- Product-specific local layout: `PlainLink`, `SectionLabel`, `MultiLineInput`, `ImportHeroIcon`, `NoteRow`, `ToggleRow`, `CustomFieldRow`, `DetailTextBox`, `DetailFieldBox`, `ImportReviewRow`, `ImportResultRow`, `EditFieldOverlay`, `NewFolderOverlay`, `AddFieldOverlay`.

The remaining visual drift risk is no longer basic controls; it is page shell/backboard fidelity, product-specific row/card styling, and keyboard/layout behavior around local multiline fields and modals.

## Replacement Map

| Secure Notes area | Current local implementation | Recommended source of truth | Refactor action |
| --- | --- | --- | --- |
| Window background and keyboard defocus | Custom background image plus scattered focus sink/touch areas | `BaseWindow` and `BasePage` from `ui/ui/widgets/base-page.slint`; official background assets from `ui/ui/images/` | Replace root shell with KeyOS base primitives where SDK exposure allows. Use the official public light/dark KeyOS background assets instead of the local reverse-engineered SVG. |
| Page headers and back/menu actions | Hand-positioned title plus `GlyphButton` | `AppHeader`, `NavHeader`, `IconButton` | Replace hand-positioned headers. Use real `chevron-left`, `ellipsis`, and action icons through `IconButton`. |
| Primary/secondary/destructive buttons | `ActionButton` with local colors/radius | `Button` | Replace with `Button` using `Importance.primary`, `secondary`, and `negative`. |
| Text inputs | `PillInput` and ad hoc error labels | `Input` and `TitledInput` | Replace single-line fields. This brings focus border, clear button, error/notice messaging, password mode, and accepted behavior into the official component. |
| Multi-line secure note fields | Custom `MultiLineInput` | No exact source found in sampled widgets | Keep custom temporarily, but restyle using `Input` token values and isolate as a single `SecureNotesTextArea` wrapper. Track as intentional divergence. |
| Search | Local `SearchInput` | `SearchInput` | Replace. Official component already supports left search icon and clear button. |
| Dropdowns for type/folder/filter/sort | Mixed local picker overlays and SDK import | `Dropdown` and `DropdownOverlay` | Replace all dropdown rows and overlays. This should fix selected-item highlight bugs and inconsistent chevrons. |
| Folder picker | Custom folder option overlay | `Dropdown`, or `FolderList` if folder icon rows are needed | Use `Dropdown` for compact form assignment. Use `FolderList` only for a dedicated folder browser if introduced later. |
| Boolean fields and PIN toggles | Custom `InlineToggle` / `ToggleRow` | `Switch` | Replace. Official switch has the correct sizing, knob math, colors, and animation. |
| Ellipsis menu | `MenuOverlay` and `MenuItem` | `OverlayMenu`, `PopupMenu`, `PopupMenuItem` | Replace. This directly addresses the repeated icon positioning/layout bug. |
| Bottom filter sheet | Custom `FilterSheetOverlay` | `Drawer` plus `Dropdown` rows | Rebuild as a KeyOS drawer. Dismiss keyboard before opening. Use official dropdown rows inside. |
| Modal text entry for new folder | `NewFolderOverlay` with manual keyboard offset | `TextInputModal` pattern from file browser, or `Modal`/`Panel` wrapper | Replace scaling/absolute positioning with translated panel offset. Buttons must remain inside the panel. |
| Confirmation/result modals | Custom overlays | `ConfirmationModal`, `ResultModal`, `Dialog` | Replace delete confirmations and import completion screen chrome where practical. |
| Note list rows | Custom `NoteRow` | No exact neutral row source found in sampled widgets; Vault uses colorful `LineCard` intentionally diverged from | Keep custom list row, but derive spacing, icon slots, text styles, border, and touch target from KeyOS tokens. Record divergence: Secure Notes intentionally avoids Vault's colorful seed cards and hides secret details on list. |
| Import review/result rows | Custom rows | Same as note list row plus `Switch` and `Icon` | Keep row structure but replace switches/icons with official components. |
| Icon rendering | Local `Image` loads from `resources/icons` | `Icon` / `Images.icon(name, size)` | Prefer official icon pipeline if SDK exposes required names. Otherwise keep local designer SVGs with an asset manifest and render near native size inside fixed slots. |
| Fades for clipped scroll content | Plain clipped scroll frame, no active fades | `KeyOS/ui/ui/widgets/shield.slint` for future full-shell migration only | Do not use Shield assets as content-width overlays. The production route is to adopt the full Shield/backboard shell. Until then, no fade is better than an incorrect rim/wedge/overlay. |

## Intentional Divergences

These are product decisions, not accidental visual drift:

- Home rows should be neutral mature list cards, not Vault's colorful seed cards.
- Home rows should show record name/type only, not secret details.
- Import lives in the ellipsis menu, not the main list affordance.
- PIN-required records should show a lock indicator in list rows and route through a restricted PIN gate.
- Secure Notes must support Bitwarden-compatible item types and fields rather than Vault seed-only workflows.
- Secure Notes needs fuzzy duplicate review during import, which Bitwarden itself does not provide.

## Implementation Order

1. Establish a small local adapter layer, for example `secure-notes/ui/components/`, for Secure Notes-specific wrappers around official widgets.
2. Replace theme constants with KeyOS theme exports. Remove local hardcoded teal/grey values where official `Palette` and `Importance` values exist.
3. Replace root shell/header/buttons/search/dropdowns/switches first. These are low-risk and remove the highest visible drift.
4. Replace ellipsis menu and filter sheet with `OverlayMenu`/`PopupMenu` and `Drawer`.
5. Replace new-folder/edit-field/delete/import-complete popups with modal primitives.
6. Rework edit/detail form rows around `TitledInput`, `Input`, `Switch`, and a single custom multiline wrapper.
7. Normalize list/import rows to official icon and spacing primitives while preserving the neutral Secure Notes list design.
8. Revisit keyboard behavior after official inputs/modals are in place. Remove hardcoded field-y focus offsets and centralize any remaining keyboard workaround.
9. Rebuild and run simulator only after the source refactor compiles locally or after a focused stage is ready for user review.

## Keyboard Refactor Notes

The current app has hardcoded focus offsets such as `focus-edit-field(204px, 56px)` and temporary spacer values such as `edit-keyboard-spacer`. Those should be deleted rather than patched.

Target behavior:

- Official `Input`/`TitledInput` owns text focus, accepted, clear, error, and password reveal behavior.
- A single screen-level coordinator handles keyboard avoidance only for text fields.
- Non-text controls never trigger keyboard avoidance.
- Opening menus, filters, dropdowns, or modals first dismisses keyboard focus.
- Text-entry modals translate above the keyboard; they do not scale.

## Open Questions Before Code Refactor

- Does the Foundation SDK version bundled with the public developer tooling expose all sampled widgets through `@ui/widgets.slint`, or must Secure Notes vendor a subset?
- If vendoring is required, should copied GPL UI files live under a clearly named `secure-notes/ui/upstream_keyos/` folder with source headers intact?
- Should the app use the public KeyOS icon names through `Images.icon`, or keep the existing local Figma-selected SVG manifest until Foundation designers provide app-specific icon names?
- Is Secure Notes expected to remain a standalone prototype repo, or eventually move into the public KeyOS app tree where imports can become native and updates propagate automatically?

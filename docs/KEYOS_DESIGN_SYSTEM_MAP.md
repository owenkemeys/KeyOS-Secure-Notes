# KeyOS Design System Map For Secure Notes

This document maps the KeyOS production design-system Figma file to the public SDK Slint components that Secure Notes should prefer.

It is intentionally source-oriented: use the Figma file for design intent, then verify the installed SDK component API before implementation. Do not recreate UI from screenshots when a matching SDK component exists.

## Sources

- Figma design intent: [Design System KeyOS](https://www.figma.com/design/tibcuJ3JvgQQIirfuvsc32/Design-System---KeyOS?node-id=1704-20431)
- Installed SDK import root used by Secure Notes: `@ui/...`
- Current local SDK snapshot for inspection: `Codex_Working/sdk_ui_snapshot/`
- Existing Secure Notes UI audit: `docs/UI_COMPONENT_SOURCE_MAP.md`

Top-level Figma pages observed:

| Page | Node | Primary use |
| --- | --- | --- |
| Shield / Backboard | `1704:20431` | App shell, backgrounds, top bar, shield content mask, content faders |
| Cards | `9:10138` | Card/list item geometry, colored card variants, card pills |
| Base Elements | `2284:206594` | Lines Prime pattern, grabber/home indicator, divider, scrollbar, amount/address primitives |
| Buttons | `9:10140` | Button variants, filter button, lock button, link, chips, labeled switch |
| Form Elements | `242:861` | Input, search, dropdown, dropdown list/options, toggle, checkbox/radio |
| WIP / Backlog | `9196:7967` | Not a source of truth unless design confirms it |

## Shell And Backboard

The `Shield / Backboard` page is the source of truth for the app frame:

- Prime screen size is `480 x 800`.
- Status bar occupies the top `40px`.
- Top bar is `64px` high and begins around `y=44`.
- Page content shell begins around `x=4`, `y=108`, with usable content about `472 x 672`.
- Standard page content inset is `24px`, giving content rows around `424px` wide.
- The production shell uses official background assets plus a shield/backboard mask, not only a flat full-screen background image.
- Scroll clipping is handled by the production Shield alpha-mask pattern:
  - `KeyOS/ui/ui/widgets/shield.slint`
  - shield background: `KeyOS/ui/ui/images/shield-bg__64-64-96-64.png`
  - dark shield background: `KeyOS/ui/ui/images/shield-bg-dark__64-64-96-64.png`
  - top mask: `50px`, `KeyOS/ui/ui/images/shield-top-alpha-mask.png`
  - bottom mask: `100px`, `KeyOS/ui/ui/images/shield-bottom-alpha-mask.png`
- Screens with fixed bottom action buttons should still use the same horizontal bottom mask behavior. Do not use the Figma `Bottom Button` SVG as a grey overlay.
- Faders should appear only when content is actually clipped; they are not decorative bands.

Secure Notes currently uses plain clipped scroll frames. A partial content-width port of the production KeyOS Shield component was rejected after simulator review because Shield is a full shell/backboard treatment, not a generic local fade strip. It produced visible rim/wedge artifacts and incorrect clipping. Future Shield parity should be done only as a full page-shell migration.

## Buttons And Action Controls

Figma page: `Buttons`.

Production design intent:

- Primary/secondary/tertiary/notice/negative button variants are distinct component states.
- Standard action button height is `60px`.
- Standard full-width action button in Prime content is `424px` wide.
- Standard radius is visually around `24px`.
- Filter button is a `56 x 56` control with states: default, pressed, transparent, active, tertiary, disabled.
- Lock button is a compact `28 x 28` state icon.
- Links are text controls, not improvised rectangles.

SDK mapping:

| Design element | SDK component |
| --- | --- |
| Primary/secondary/negative action buttons | `@ui/button.slint::Button` |
| Icon-only controls | `@ui/icon_button.slint::IconButton` |
| Filter button shell | `IconButton` adapter with `56px` control size and active styling |
| Links | `@ui/link.slint::Link` |

Secure Notes should keep product-specific labels and callbacks local, but the button rendering should be SDK-backed.

## Forms

Figma page: `Form Elements`.

Production design intent:

- Single-line inputs are `56px` high.
- Active input uses the KeyOS brand/teal border.
- Input states include default, disabled, filled, active, error, notice, seed, and buttons.
- Search is a `56px` high input variant with search icon and filled/active states.
- Dropdown trigger is `56px` high and has a matching dropdown list/options component.
- Toggle is `56 x 28`.
- Dropdown option states include default, active, pressed, and section break.

SDK mapping:

| Design element | SDK component |
| --- | --- |
| Single-line input | `@ui/input.slint::Input` |
| Multi-line text entry | `@ui/textarea.slint::TextArea` |
| Search | `@ui/search.slint::Search` |
| Dropdown | `@ui/dropdown.slint::Dropdown` |
| Toggle/switch | `@ui/switch.slint::Switch` |

Implementation notes:

- Use SDK `Input` for the visual state, but preserve explicit Done-key dismissal where the app requires it. The SDK input emits `submitted` but does not clear focus by itself.
- Use SDK `TextArea` for multiline styling where possible; if Done-key behavior is incompatible, wrap it and centralize the workaround.
- Do not attach keyboard avoidance to toggles, dropdowns, buttons, or filter controls.

## Menus, Sheets, And Dialogs

SDK mapping:

| Design element | SDK component |
| --- | --- |
| Popup menu container | `@ui/menu.slint::Menu` |
| Popup menu rows | `@ui/menu_item.slint::MenuItem` |
| Bottom sheet shell | `@ui/sheet.slint::Sheet`, or local drawer if custom content is required |
| Confirmation/result dialog | `@ui/dialog.slint::Dialog` |

The installed SDK `Sheet` currently includes demo-like body content. For Secure Notes, use its geometry as reference, but keep a local drawer body if the SDK component cannot host arbitrary children cleanly.

The ellipsis menu should not be hand-positioned row-by-row. Use `Menu`/`MenuItem` so row height, text baseline, dividers, and trailing icon alignment match the SDK.

## Cards And Rows

Figma page: `Cards`.

Production design intent:

- Standard full card width in Prime content is `424px`.
- Standard full card height in the design-system demo is `148px`; compact card examples are `100px`.
- Card background variants include Blue, Teal, Darkcopper, Lightcopper, Pine, Green, Purple, Darkgrey.
- Card subcomponents include `.assets/CardElements`, `.assets/CardPill`, `.assets/AccountType`, and `.assets/Icon`.

Secure Notes intentional divergence:

- Secure Notes should not use Vault-style colorful cards for every record.
- Home rows should remain neutral and mature.
- Home rows should show only item name/type, not secret data.

Even with the neutral design, rows should still borrow production card geometry:

- `424px` row width inside content inset.
- Stable row heights and touch targets.
- Official icon-button/icon rendering where possible.
- Proper fader/masking behavior when rows are clipped by fixed bottom controls.

## Base Elements

Figma page: `Base Elements`.

Important reusable primitives:

- `Lines Prime` pattern has Grey/White and Looser/Closer variants.
- `Divider` is a simple `1px` line.
- `.assets/Grabber` includes the bottom-sheet grabber/home-indicator patterns.
- `assets/Scrollbar` is present as a reference for constrained scrollable areas.

Secure Notes should avoid rebuilding gradients, bands, or fake masks when the Base Elements page already defines the intended primitive.

## Current Secure Notes Refactor Target

Highest-value remaining migrations:

1. Replace local `ActionButton` with SDK `Button`.
2. Replace local `GlyphButton` and row icon buttons with SDK `IconButton`.
3. Replace local `PillInput`, `MultiLineInput`, and `SearchInput` with SDK `Input`, `TextArea`, and `Search` adapters.
4. Replace local `InlineToggle`/`ToggleRow` with SDK `Switch`.
5. Keep ellipsis menu rows as direct children of SDK `Menu`; do not wrap them in an extra local layout, because `Menu` already owns child layout and height calculation.
6. Rework filter sheet against SDK drawer/menu/dropdown primitives.
7. If Shield/backboard parity is revisited, migrate the full page shell to the production Shield model. Do not use Shield assets as content-width overlays or generated background-cover fade derivatives.
8. Keep neutral Secure Notes rows, but normalize them to production card/content measurements.

## Rule Of Thumb

Use this decision order for future KeyOS UI work:

1. Find the UI element in the Figma design-system page.
2. Find the corresponding installed SDK component under `@ui`.
3. Wrap the SDK component in an app-specific adapter only when callback shape or product behavior differs.
4. Keep a local custom component only when there is no SDK equivalent or the product intentionally diverges.
5. Record every intentional divergence in this document or `UI_COMPONENT_SOURCE_MAP.md`.

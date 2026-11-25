# KatWork fork of PhotoCraft

This repository is the KatWork fork of PhotoCraft (https://github.com/storytold/photocraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on PhotoCraft by the ArtCraft team (https://github.com/storytold/photocraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `e22facf45b0e7f0fbeb3b439ed44a950da9783b4`
- Upstream `storytold/photocraft` head at that time: `e22facf45b0e7f0fbeb3b439ed44a950da9783b4`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): plugins feature, on by default` | draft, not opened (in the fork PR body) |

## The `plugins` feature

The WebAssembly plug-in host (`photocraft-plugins`, the wasmi interpreter) is the Cargo feature `plugins`, on by default
in `photocraft-engine-core` (where the host and the `plugin.*` commands live), `photocraft-engine`, `photocraft-ui-egui`
(Filter › Plug-ins), `photocraft-automation`, the CLI, the web app and the desktop app, so an upstream build is unchanged.
KatWork builds the CLI and the web app with `--no-default-features` (no other feature): `photocraft-plugins` and `wasmi`
are then not in either graph (`scripts/katwork-profile.sh`, CI job `katwork-profile`); every `plugin.*` command is unknown,
`plugin.install` with inline `data` or `dataBase64` included; a smart filter a document names as `plugin.run` draws
nothing, as when its plug-in isn't installed, and stays in the document; the Plug-ins preference folder is not read.

The CLI's other default, `gpu`, is left out by the same `--no-default-features` (KatWork never starts a Craft app with
`--gpu`). `heif` stays off in both builds (it is not a default). This fork also removes the `data-cargo-features="heif"`
that upstream's `apps/photocraft-web/index.html` sets, so the fork's own web build equals KatWork's (HEIF off); upstream's
official web build turns HEIF on there. `apps/photocraft-web-boot` (the second web crate, which loads the wasm in parts)
never held the host; its rows in `scripts/katwork-profile.tsv` are vacuous and say so.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/photocraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.photocraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The app icon's attribution (`ATTRIBUTION.md` row, `assets/app-icon/LICENSE.txt`, `assets/app-icon/README.md`,
  `docs/releasing.md`): the icon is original artwork by the project owner, drawn in ArtCraft; these author and provenance lines
  are licence and attribution notices. The icon itself is kept (R43) and shows no mark.
- Per-user folders the app and the suite use (`Documents/ArtCraft/Agents/PhotoCraft`, the `ArtCraft/flags.toml` suite flags file,
  the Windows `ArtCraft\PhotoCraft` program folder) in `apps/photocraft`, `crates/automation`, `docs/`: paths, kept like the
  installer paths; renaming them would move existing users' files.
- The separate ArtCraft Launcher program (its AI-agents switch, its nightly channel and profiles, its version folder in the
  Flatpak manifests, its `craft-ai-mcp` gateway entry named `ArtCraft`) in code comments, tests, nightly-channel messages and docs:
  it is another program; KatWork builds no nightly and never starts the launcher or its switch.
- The theme id `craft.artcraft` (`crates/engine-core/src/prefs*.rs`, `crates/ui-egui/src/theme.rs`,
  `crates/ui-egui/src/app/framework.rs`): an identifier of a theme defined by the shared `craft-libs` theme crate (a git
  dependency), stored in users' preferences; renaming it needs a change in `craft-libs` and would reset saved themes.
- Upstream CI and tooling (`.github/workflows/*.yml` runner labels and secret names, `.config/nextest.toml` and test comments
  about the upstream `artcraft-k8s` runners, `docs/user/AGENTS.md`'s website doc checker): unchanged upstream CI.
- The "Based on" line inside code (`crates/ui-egui/src/dialogs/credits.rs`, `BASED_ON`): the About window's plain-text line.
- Not in this repository: the shared PDF writer (`craft-pdf`, a git dependency) writes the XMP namespace
  `http://ns.<the ArtCraft website>/photocraft/spot-plates/1.0/` into exported PDFs with spot plates. It is an identifier that
  converters match, not a link shown to anyone; changing it needs a change in `craft-libs`. The test that checks it now reads
  the constant from `craft-pdf` instead of repeating the URL.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.photocraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.photocraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-nightly-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-nightly.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-nightly.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-nightly.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/photocraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/icons/arrow-left-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/circle-dashed.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/ellipsis.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/file-plus.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/folder-plus.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/grid-2x2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/history-brush.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/house.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/link-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/message-square.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/panel-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-combine.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-exclude.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-intersect.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-merge.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-new-layer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/path-op-subtract.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pipette.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/rectangle-horizontal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/rectangle-vertical.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/redo-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/spirit-level.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/square-dashed-mouse-pointer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/square-dashed.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/triangle-alert.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/type.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/vector-shape.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/x.svg`: kept (viewed: no ArtCraft mark)
- `docs/images/clone-source/cross-document.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/clone-source/offsets-after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/clone-source/offsets-before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/clone-source/opacity-flow-after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/clone-source/opacity-flow-before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/clone-source/source-edge.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/cpu-pixel-alignment/after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/cpu-pixel-alignment/before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/embedded-vector-editor-en.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/embedded-vector-tabs-linux.png`: kept: screenshot of the upstream VectorCraft build (viewed); its app bar's Discord button is painted over with the bar's colour
- `docs/images/multi-channel-editing/cmyk.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/curves-choices.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/curves.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/drawing.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/gallery-channel-preview-after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/gallery-channel-preview-before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/great-wave.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/lab.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/levels.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/mask-view-after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/mask-view-before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/multi-channel-editing/rgb.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-adjustments.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-demo.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-export-light.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-filters.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-layer-styles.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-masks.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-transform.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-type.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/photocraft-vector.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/preset-invalid-choice-after.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/preset-invalid-choice-before.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.svg`: kept (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept (viewed: no ArtCraft mark)

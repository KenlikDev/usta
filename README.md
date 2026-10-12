# Usta

Usta is an open-source desktop 3D creation tool focused on modeling, with a possible future path toward a game engine and tools for architects, engineers, and designers.

> **Current status:** experimental foundation / technology spike. The current branch contains a Bevy-based scene workbench with one or more cube objects, click selection, keyboard transform editing, and local JSON save/load. It is not yet a complete 3D editor.

## Project documents

- [Product vision and candidate MVP](docs/PRODUCT_VISION.md)
- [Proposed architecture overview](docs/architecture/overview.md)
- [UI/rendering technology spike](docs/spikes/2026-10-bevy-scene-workbench.md)
- [ADR 0001: Bevy prototype baseline](docs/architecture/decisions/0001-bevy-scene-workbench-spike.md)

These documents distinguish observed repository facts from proposed direction. The Bevy choice is limited to the prototype and is not the final application architecture.

## Run the prototype

Requirements: stable Rust toolchain and platform dependencies required by Bevy/winit and the graphics backend. See [the Rust CI workflow](.github/workflows/rust.yml) for Linux build libraries.

From the repository root:

```bash
cargo run
```

## Prototype controls

**Viewport**
- **Left-click cube:** select the cube; click the ground to clear selection.
- **Right mouse button + drag:** orbit the camera.
- **Middle mouse button + drag:** pan the view.
- **Mouse wheel:** zoom.
- **Home:** reset the camera.

**Selected cube**
- **Arrow keys:** move along world X/Z.
- **Page Up / Page Down:** move along world Y.
- **Q / E:** rotate around world Y.
- **Hold Shift:** use a larger movement step.
- **N:** create a new cube beside the selected object (or near the origin if none is selected).
- **Delete:** remove the selected object. The scene may be empty; press **N** to add a new cube.

**Project file**
- **Ctrl+S:** save editable objects to `usta_scene.json` in the current working directory.
- **Ctrl+O:** load `usta_scene.json`. The file must exist and pass schema and transform validation.
- Saving/loading currently persists editable cube objects only. The camera, lights, ground, editor layout, selection, and UI state are not part of the scene document.
- Scene files are local user data and are ignored by Git.

The inspector displays the selected object's position, Y rotation, and scale. The outliner remains a simplified text panel.

## Scene document format

Scene files use JSON with `schema_version: 1`, stable object IDs, primitive kinds, and explicit translation, quaternion rotation ([x, y, z, w]), and scale arrays.

The loader validates the entire file before replacing the live scene. It rejects unsupported schema versions, duplicate/empty IDs, non-finite transforms, zero-length quaternions, near-zero scale components, malformed JSON, and documents above the prototype object limit. Empty scenes are valid. Invalid load attempts leave the current scene in place and display an error.

The format currently supports the `cube` primitive. Schema changes must increment the version and be covered by tests.

## Current limitations

- No undo/redo, mesh editing, materials editor, production-grade hierarchy, or asset pipeline.
- No file picker or user-configurable project path; Ctrl+S / Ctrl+O use the fixed filename above.
- The app and runtime pointer/graphics behavior still require manual verification on the target desktop OS.
- The final graphics/UI stack and supported OS list have not yet been decided.

## Quality checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

These commands are configured in GitHub Actions. A passing CI run confirms formatting, compilation, and tests on the runner; it does not substitute for checking graphics and pointer interactions on a real desktop.

## License

Usta is distributed under the [MIT License](LICENSE).

# Usta

Usta is an open-source desktop 3D creation tool focused on modeling, with a possible future path toward a game engine and tools for architects, engineers, and designers.

> **Current status:** experimental foundation / technology spike. The current branch contains a Bevy-based viewport prototype with basic click-selection and keyboard transform editing for one cube. It is not yet a complete 3D editor and does not persist scenes to disk.

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

The inspector displays the selected object's position and Y rotation, and a wireframe marks its selection. The outliner remains a simplified text view. The prototype only has one editable object.

## Current limitations

- No persistent scene document or save/load.
- No real multi-object hierarchy, mesh editing, materials editor, undo/redo, or asset pipeline.
- The selected cube's transform edits are transient and reset on restart.
- Runtime input/rendering still requires manual verification on the target desktop OS.

## Quality checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

These commands are configured in GitHub Actions. A passing CI run confirms formatting, compilation, and tests on the runner; it does not substitute for checking graphics and pointer interactions on a real desktop.

## License

Usta is distributed under the [MIT License](LICENSE).

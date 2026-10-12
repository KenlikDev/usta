# UI and Rendering Technology Spike — October 2026

**Status:** Prototype implemented; automated checks are being run against the latest commit. Bevy is selected only as the first prototype baseline; this is not approval of the final Usta architecture.

## Purpose

Test whether Usta can establish a native desktop window, a real 3D viewport, a ground grid, primitives, camera controls, and a minimal create/edit/save/load workflow. Keep the experiment small enough to discard or replace without committing the long-term product architecture to the engine.

## Current prototype

The branch `spike/bevy-scene-workbench` contains a Bevy-based application spike with:

- a native window, 3D camera, cube primitives, ground plane, directional light, and gizmo grid;
- right-drag orbit, middle-drag pan, mouse-wheel zoom, and Home-to-reset controls;
- click selection for cubes; clicking the ground clears selection;
- a selection wireframe and inspector readout for position, Y rotation, and scale;
- keyboard transform editing for selected cubes;
- keyboard creation (N) and deletion (Delete) of cube objects, including support for an empty scene;
- a renderer-independent `src/document.rs` module for a versioned scene document;
- Ctrl+S / Ctrl+O save/load for `usta_scene.json` in the current working directory;
- schema and transform validation, atomic replacement during save, and tests for serialization, file round-trip, malformed data, duplicate IDs, unsupported schema versions, and invalid transforms;
- a GitHub Actions workflow for Rust formatting, compilation checks, and tests.

The persisted scene currently includes **editable cube objects only**. Camera, ground, lights, editor layout, current selection, and UI state are transient and are not stored in the document.

## Document boundary

The scene document deliberately contains no Bevy entity IDs, asset handles, UI components, or GPU state. Each object has a stable string ID, a primitive kind, translation, quaternion rotation in [x, y, z, w] order, and scale.

The current JSON schema uses version 1 and allows up to 256 editable objects; empty scenes are valid. The loader fully parses and validates the file before changing the live scene; on failure the existing scene is preserved and the UI reports the problem. This is a small persistence slice, not yet a migration framework or production file format.

## Candidate comparison

This comparison is based on official documentation, not on measured build or runtime performance in this repository.

| Candidate | Strength for this spike | Main risk / cost | Current assessment |
|---|---|---|---|
| **Bevy 0.20.0** | Integrated native windowing, 3D mesh/material rendering, UI, input, gizmos, and mesh-picking example. | Usta becomes coupled to a game engine's ECS/rendering model; APIs evolve quickly. | Useful for proving the first 3D interaction loop quickly. Keep the persistent scene/document model independent from Bevy entities. |
| **egui 0.36.2 + wgpu** | Strong fit for tool panels and custom editor UI; official examples demonstrate embedding 3D rendering with WGPU callbacks. | Usta must own more rendering integration and graphics resource lifecycle. | Credible alternative if Bevy's engine coupling becomes a problem. Do a second spike before finalizing. |
| **Slint 1.18.1 + WGPU** | Declarative UI and multiple desktop backends; documented external WGPU rendering integration. | The documented WGPU 30 integration is feature-gated as `unstable-wgpu-30`; verify API stability and compatibility before relying on it. | Needs a separate compatibility experiment before being considered for a renderer-heavy editor. |

Official reference material checked for this spike:

- Bevy 0.20 release announcement: https://bevy.org/news/bevy-0-20/
- Bevy 3D scene example: https://bevy.org/examples/3d-rendering/3d-scene/
- Bevy camera orbit example: https://github.com/bevyengine/bevy/blob/v0.20.0/examples/camera/camera_orbit.rs
- Bevy mesh picking example: https://github.com/bevyengine/bevy/blob/v0.20.0/examples/picking/mesh_picking.rs
- egui 0.36.2 and official integrations: https://docs.rs/crate/egui/latest
- egui custom WGPU 3D example: https://github.com/emilk/egui/blob/main/crates/egui_demo_app/src/apps/custom3d_wgpu.rs
- Slint's WGPU 30 integration notes: https://docs.rs/slint/latest/slint/wgpu_30/index.html

Bevy 0.20.0 was released on 8 October 2026. Its documentation warns that Bevy is still early-stage and that breaking changes are expected in periodic releases. This is an important long-term risk, not a reason to avoid using it for a disposable spike.

## How to run

Requirements: stable Rust toolchain, desktop platform prerequisites for Bevy/winit and the selected graphics backend.

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo run
```

Linux build prerequisites are listed in `.github/workflows/rust.yml`. The first build may take a while due to the engine and graphics dependencies.

On launch, the prototype starts with one cube. Press N to add cubes and Delete to remove the selected cube (including the last one). Select a cube, change its transform, press Ctrl+S to save, and press Ctrl+O to load the last saved scene. This uses a fixed project filename; no file picker is implemented yet.

## Validation status

The earlier viewport + object-editing prototype passed formatting, compilation, and tests in GitHub Actions: https://github.com/KenlikDev/usta/actions/runs/38267559617

The latest persistence changes require a fresh completed run. Even a green CI result does not prove pointer selection or graphics render correctly on a real desktop; those still need manual verification on a declared target OS.

## Remaining technical questions

1. Does the viewport and picking work reliably on the first supported desktop OS?
2. Can a persistent document model remain independent of Bevy entities and GPU resources as multiple object kinds are added?
3. Is Ctrl+S/Ctrl+O enough for the first user workflow, or should the next slice add a file picker and configurable path?
4. How much work is required to replace the static outliner with a real hierarchy?
5. Is the engine coupling acceptable for a general-purpose editor, or should an egui + WGPU spike be compared before expanding the architecture?

After runtime verification, compare results with a minimal egui + WGPU integration. Record the final choice in a separate ADR rather than treating this prototype choice as permanent.

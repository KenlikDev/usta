# UI and Rendering Technology Spike — October 2026

**Status:** In progress. Bevy is selected only as the first prototype baseline; this is not approval of the final Usta architecture.

## Purpose

Test whether Usta can quickly establish a native desktop window, a real 3D viewport, a ground grid, a primitive, and useful camera controls. Keep the test small enough to discard or replace without affecting the long-term product architecture.

## Current prototype

The branch `spike/bevy-scene-workbench` contains a Bevy-based application spike with:

- a native window titled "Usta — Scene Workbench (prototype)";
- a 3D camera, a cube, a ground plane, a directional light, and an editor-style grid;
- an informational overlay describing viewport controls and a static outliner/inspector placeholder;
- right-drag orbit, middle-drag pan, mouse-wheel zoom, and Home-to-reset controls;
- small unit tests for orbit-camera distance invariants;
- a GitHub Actions workflow for formatting, checking, and running Rust tests.

**Not implemented:** object selection, editable inspector fields, a persistent scene document, file save/load, mesh editing, materials UI, undo/redo, a final application shell, or packaging.

The prototype's outliner and inspector are labels only. They must not be presented as functional editor panels.

## Candidate comparison

This comparison is based on official documentation, not on measured build or runtime performance in this repository.

| Candidate | Strength for this spike | Main risk / cost | Current assessment |
|---|---|---|---|
| **Bevy 0.20.0** | Integrated native windowing, 3D mesh/material rendering, UI, input, and gizmos; official examples cover basic 3D scenes and orbit cameras. | Usta becomes coupled to a game engine's ECS/rendering model; Bevy's API evolves quickly and releases can contain breaking changes. | Best first spike for proving the real 3D interaction loop quickly. Keep scene/document data separate when implementation grows. |
| **egui 0.36.2 + wgpu** | Strong fit for tool panels and custom editor UI; official egui examples demonstrate embedding 3D rendering with WGPU callbacks. | Usta must own more rendering integration, render-to-texture/depth handling, and lifecycle boundaries. Requires an explicit graphics/UI integration prototype before it can be compared fairly. | Credible alternative if Bevy's engine coupling becomes a problem. Do a second spike before a final framework choice. |
| **Slint 1.18.1 + WGPU** | Declarative UI and multiple desktop backends; documented external WGPU rendering integration. | The documented WGPU 30 integration is currently feature-gated as `unstable-wgpu-30`; verify API stability and compatibility before relying on it. | Do not choose as first 3D integration baseline without a separate compatibility experiment. |

Official reference material checked for this spike:

- Bevy 0.20 release announcement: https://bevy.org/news/bevy-0-20/
- Bevy 3D scene example: https://bevy.org/examples/3d-rendering/3d-scene/
- Bevy camera orbit example: https://github.com/bevyengine/bevy/blob/main/examples/camera/camera_orbit.rs
- egui 0.36.2 and official integrations: https://docs.rs/crate/egui/latest
- egui custom WGPU 3D example: https://github.com/emilk/egui/blob/main/crates/egui_demo_app/src/apps/custom3d_wgpu.rs
- Slint's WGPU 30 integration notes: https://docs.rs/slint/latest/slint/wgpu_30/index.html

Bevy 0.20.0 was released on 8 October 2026. Its own documentation warns that Bevy is still early-stage and that breaking API changes are expected in periodic releases. This is an important long-term risk, not a reason to avoid using it for a disposable spike.

## How to run

Requirements: stable Rust toolchain, desktop platform prerequisites for Bevy/winit and the selected graphics backend.

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo run
```

Linux build prerequisites are listed in `.github/workflows/rust.yml`. First build may take several minutes due to the engine and graphics dependencies.

## What must be observed before deciding

1. Does a clean checkout compile on the first declared target OS?
2. Does a real viewport render the cube and grid without runtime errors?
3. Are orbit, pan, zoom, and reset reliable?
4. Can a document model live outside Bevy entities and GPU resources?
5. How much work is required to make a genuine hierarchy and inspector, rather than informational UI labels?
6. Is the engine coupling acceptable for a general-purpose editor expected to grow toward modeling and engineering workflows?

## Exit criteria

The spike is successful only if the clean build, checks, and runtime workflow have actually been executed and recorded. A code commit or a successful `cargo check` alone does not prove that the viewport works on a real machine.

After validation, compare results with a minimal egui + WGPU integration. Record the final choice in a separate ADR rather than treating this prototype choice as permanent.

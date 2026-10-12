# UI and Rendering Technology Spike — October 2026

**Status:** In progress. Bevy is selected only as the first prototype baseline; this is not approval of the final Usta architecture.

## Purpose

Test whether Usta can quickly establish a native desktop window, a real 3D viewport, a ground grid, a primitive, and useful camera controls. Keep the test small enough to discard or replace without affecting the long-term product architecture.

## Current prototype

The branch `spike/bevy-scene-workbench` contains a Bevy-based application spike with:

- a native window titled "Usta — Scene Workbench (prototype)";
- a 3D camera, a cube, a ground plane, a directional light, and an editor-style grid;
- right-drag orbit, middle-drag pan, mouse-wheel zoom, and Home-to-reset controls;
- mesh picking for left-click selection of the cube; clicking the ground clears selection;
- a wireframe highlight and a small inspector that displays the selected object's position and Y rotation;
- keyboard transform editing for the one cube (arrow keys, Page Up/Down, Q/E, with Shift for larger translation steps);
- unit tests for orbit-camera invariants and transform commands;
- a GitHub Actions workflow for formatting, compilation checks, and tests.

**Still not implemented:** a persistent scene/document model, file save/load, multiple independently editable objects, a true hierarchy panel, direct numeric inspector editing, mesh editing, materials UI, undo/redo, final application packaging, or a release process.

The outliner remains a simplified text view, not a functional hierarchy. This prototype should not be presented as a complete editor.

## Candidate comparison

This comparison is based on official documentation, not on measured build or runtime performance in this repository.

| Candidate | Strength for this spike | Main risk / cost | Current assessment |
|---|---|---|---|
| **Bevy 0.20.0** | Integrated native windowing, 3D mesh/material rendering, UI, input, gizmos, and mesh-picking example. | Usta becomes coupled to a game engine's ECS/rendering model; APIs evolve quickly. | Best first spike for quickly proving a 3D interaction loop. Keep the persistent scene/document model independent from Bevy entities. |
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

Linux build prerequisites are listed in `.github/workflows/rust.yml`. First build may take several minutes due to the engine and graphics dependencies.

## What must be observed before deciding

1. Does a clean checkout compile on the first declared target OS?
2. Does a real viewport render the cube and grid without runtime errors?
3. Are orbit, pan, zoom, selection, and keyboard transforms reliable?
4. Can the persistent document model live outside Bevy entities and GPU resources?
5. How much work is required to make a genuine hierarchy and editable inspector?
6. Is the engine coupling acceptable for a general-purpose editor expected to grow toward modeling and engineering workflows?

## Exit criteria

The spike is successful only if the clean build, checks, and runtime workflow have actually been executed and recorded. A code commit or a successful `cargo check` alone does not prove that the viewport or pointer interactions work on a real machine.

After validation, compare results with a minimal egui + WGPU integration. Record the final choice in a separate ADR rather than treating this prototype choice as permanent.

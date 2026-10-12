# ADR 0001: Use Bevy for the first 3D workbench spike

- **Date:** 2026-10-12
- **Status:** Accepted for the prototype only; final product stack remains undecided.
- **Scope:** Technology experiment on `spike/bevy-scene-workbench`.

## Context

Usta needs evidence about the effort required to host a desktop window, render a real 3D viewport, and support camera interaction. The repository did not contain an application or confirmed technology stack. A small proof-of-concept should expose integration risks before a production architecture is selected.

## Decision

Use **Bevy 0.20.0** as the first prototype baseline. This is a bounded experiment to test a 3D interaction loop, not a decision to make Bevy's ECS or world representation the canonical Usta document model.

The prototype should keep the scene/document architecture open. No persistence format, engine plugin model, or permanent entity/component schema is decided by this ADR.

## Alternatives considered

### egui 0.36.2 + WGPU

Advantages:
- Tool-oriented UI and panels are a natural fit for editor-style applications.
- Official egui examples document a WGPU path for embedded 3D rendering.

Costs:
- Usta would need to implement and maintain more of the custom viewport rendering and graphics resource lifecycle.
- A fair evaluation needs a small, equivalent viewport prototype.

This remains the principal alternative to compare after the Bevy spike.

### Slint 1.18.1 + WGPU

Advantages:
- Declarative UI and documented cross-platform desktop backends.

Costs:
- The documented WGPU 30 integration is behind the `unstable-wgpu-30` feature flag and may change between minor releases.
- Needs a separate integration experiment before being selected for a renderer-heavy application.

### Implement a custom windowing and rendering layer

This provides maximum control but requires more foundational work before product behavior can be validated. It is not justified until framework-based spikes reveal blocking constraints.

## Evidence

- Bevy 0.20.0 was released on 2026-10-08: https://bevy.org/news/bevy-0-20/
- The official Bevy examples include a basic 3D scene and mouse-driven orbit camera: https://bevy.org/examples/3d-rendering/3d-scene/ and https://github.com/bevyengine/bevy/blob/main/examples/camera/camera_orbit.rs
- Bevy's crate documentation warns that the project is early-stage and that breaking changes may occur in periodic releases: https://docs.rs/crate/bevy/latest
- egui's official docs describe native integration and a custom WGPU 3D example: https://docs.rs/crate/egui/latest and https://github.com/emilk/egui/blob/main/crates/egui_demo_app/src/apps/custom3d_wgpu.rs
- Slint's WGPU 30 integration uses an unstable, feature-gated API: https://docs.rs/slint/latest/slint/wgpu_30/index.html

This is documentation-based assessment, not a measured comparison. At the time this ADR was written, the Usta prototype had not yet been built or run.

## Consequences

Positive:
- The first prototype can focus on whether Usta's core 3D interaction loop is feasible.
- Rendering, camera, light, and basic UI infrastructure are provided by one framework for the experiment.

Negative:
- The prototype will inherit Bevy's release cadence and ECS-centric model.
- A successful Bevy prototype does not prove Bevy is the best long-term editor framework.
- The first UI panels may be simpler than the needs of a mature modeling application.

## Revisit criteria

Revisit this ADR after:
1. a clean build and runtime test of this branch on the first declared desktop platform;
2. verifying that the camera and viewport interactions work;
3. estimating the cost of a true object hierarchy and editable inspector;
4. producing or evaluating an equivalent egui + WGPU spike;
5. deciding whether the canonical scene/document model can remain independent of renderer-specific state.

The final stack choice must be recorded in a separate ADR with observed results and trade-offs.

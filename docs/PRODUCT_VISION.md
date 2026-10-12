# Usta Product Vision

> **Status: Draft proposal — not yet approved.** This document separates verified facts about the current prototype branch from product hypotheses that still need validation.

## 1. Product direction

Usta is intended to become an open-source, desktop-first 3D creation tool for modeling and scene creation, with a possible future path toward a game engine and specialist workflows for architects, engineers, and designers.

The first release should prove one small, complete workflow rather than attempt to deliver a full 3D suite and game engine at once.

## 2. Verified state of the prototype branch

The repository began as a README/license/gitignore scaffold. The current `spike/bevy-scene-workbench` branch now contains a small, experimental implementation:

- `Cargo.toml` uses Rust/Cargo, pins Bevy 0.20.0 for this prototype, and adds Serde/JSON serialization. This is a bounded experiment, not a final technology commitment.
- `src/main.rs` provides a desktop viewport with cube primitives, click selection, keyboard transform editing, camera controls, and a minimal inspector/outliner presentation.
- `src/document.rs` defines a renderer-independent, versioned scene document and JSON persistence, with validation and unit tests.
- Ctrl+S and Ctrl+O save/load editable cube objects through `usta_scene.json` in the current working directory.
- GitHub Actions runs Rust formatting, compile checks, and tests on Ubuntu Linux.

Important limits remain: the project format currently stores cube objects only; camera, light, ground, selection, and UI state are transient; there is no file picker, production hierarchy, mesh editor, or undo/redo. The runtime viewport and save/load interactions still require manual verification on a declared desktop platform. The final supported OS and long-term UI/rendering stack have not been decided.

## 3. Initial product hypothesis

**Proposed first audience:** independent 3D creators, technical users, and indie developers who want a straightforward desktop tool for assembling and saving simple 3D scenes.

**Proposed first problem to solve:** make it possible to create a basic scene, inspect it in a 3D viewport, manipulate a few objects, save the scene, and open it again reliably.

This audience and problem are hypotheses to validate. The broader architecture/engineering audience may have different requirements and should not dictate the first MVP without further product discovery.

## 4. Candidate MVP: 3D Scene Workbench

The MVP should be small enough to build and verify end-to-end.

### Candidate user workflow

1. Launch Usta and create a new scene.
2. See a 3D viewport with a visible ground grid and an understandable camera view.
3. Add at least one basic primitive, such as a cube.
4. Select the object and adjust its transform using a documented interaction.
5. Save the scene to a local project file.
6. Close or reset the document, reopen the file, and recover the scene.

### Candidate acceptance criteria

The MVP can be considered complete when all of these are verified:

- A clean checkout can be built using documented commands on at least one explicitly supported desktop platform.
- The application launches without requiring network access for its core workflow.
- The user can create a scene, add a primitive, navigate the viewport, select the object, and change its transform.
- Saving and reopening a scene preserves object identity, geometry references, and transforms within the limits of the chosen file format.
- Invalid or unsupported project files produce a useful error instead of silently discarding data or crashing.
- Automated tests cover document operations and serialization; a repeatable smoke test covers application startup and the primary workflow.
- The README accurately describes what is implemented and how to run it.

These criteria are proposals. They should be revised after the platform, interaction model, and technical approach are selected.

## 5. Goals and non-goals

### Initial goals

- Desktop-first, local-first operation.
- A clear and predictable workflow for simple 3D scenes.
- Separation between scene data, presentation, and rendering.
- A versioned project-file format with documented compatibility behavior.
- Incremental delivery with reproducible builds and tests.
- An open-source project with clear provenance for third-party code and assets.

### Explicit non-goals for the first MVP

- A complete professional CAD/BIM suite.
- A production-ready game engine with physics, scripting, animation, audio, and deployment pipelines.
- Advanced sculpting, procedural node graphs, simulation, collaboration, or cloud accounts.
- Compatibility claims for platforms or file formats that have not been tested.

These features may be considered later, but none should be represented as current capability.

## 6. Product principles

1. **Prove a complete workflow early.** Prefer a small, usable vertical slice over disconnected subsystems.
2. **Keep the document authoritative.** The editable scene should not exist only as transient renderer state.
3. **Make persistence trustworthy.** Saving and reopening must be a first-class workflow, not an afterthought.
4. **Separate stable concepts from replaceable technology.** The product's scene model should not unnecessarily depend on one UI or graphics backend.
5. **Keep claims evidence-based.** Do not describe features, supported platforms, performance, or test status as complete until verified.
6. **Expand through validated needs.** Architecture/engineering and game-engine functionality should be added when the initial workflow is solid and their requirements are understood.

## 7. Decisions needed before implementation expands

- Which operating system is the first supported target?
- Which workflow best represents the first user: general 3D scene creation, game-asset preparation, or another narrow use case?
- Which desktop UI/windowing and rendering approach should become the long-term choice? The current Bevy implementation remains a prototype baseline.
- How should the current schema-v1 JSON format evolve, and what compatibility/migration policy should future versions follow?
- Which primitive types and object-manipulation interactions are essential for the MVP?
- What minimum hardware and performance target should be tested?

Record consequential technical choices as Architecture Decision Records (ADRs), including the reason for the choice and alternatives considered.

## 8. Definition of the next milestone

The **Foundation** milestone is in progress, not complete. The current branch has a buildable application prototype, a basic CI workflow, and a first scene editing/save-load path in code. The milestone should not be marked complete until:
- the current commit passes formatting, compile checks, and unit tests;
- the viewport and pointer controls have been run manually on the first declared desktop platform;
- Ctrl+S/Ctrl+O have been verified interactively, including an invalid-file case;
- the MVP scope and long-term technology choice are reviewed with measured evidence.

The goal remains a small, reliable vertical slice rather than feature completeness.

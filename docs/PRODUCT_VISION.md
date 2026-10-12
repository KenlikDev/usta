# Usta Product Vision

> **Status: Draft proposal — not yet approved.** This document separates the current repository facts from hypotheses proposed to help define the first usable version of Usta.

## 1. Product direction

Usta is intended to become an open-source, desktop-first 3D creation tool for modeling and scene creation, with a possible future path toward a game engine and specialist workflows for architects, engineers, and designers.

The first release should prove one small, complete workflow rather than attempt to deliver a full 3D suite and game engine at once.

## 2. What is known today

The current repository establishes only the following:

- The README describes Usta as an open-source desktop 3D creation tool, with modeling, a future game engine, and tools for architects, engineers, and designers.
- The repository includes an MIT license.
- The repository has not yet established an application implementation, build system, supported operating systems, or a confirmed technical stack.
- Rust-related entries in `.gitignore` are a clue, not proof that Rust has been selected.

These are repository observations, not claims about work that might exist outside this repository.

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
- Which language, UI toolkit, rendering backend, and windowing stack should be used?
- What project-file format should be used, and how will schema migrations work?
- Which primitive types and object-manipulation interactions are essential for the MVP?
- What minimum hardware and performance target should be tested?

Record consequential technical choices as Architecture Decision Records (ADRs), including the reason for the choice and alternatives considered.

## 8. Definition of the next milestone

The next milestone is **Foundation**, not feature completeness. It should end with:
- a reviewed MVP scope;
- a documented and evidence-based technology decision;
- a minimal, buildable desktop application;
- a basic automated CI check;
- one small end-to-end proof that the selected UI can host a 3D viewport.

Do not mark this milestone complete until the build and checks have actually run.

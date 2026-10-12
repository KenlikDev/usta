# Usta Architecture Overview

> **Status: Draft proposal — not yet approved.** This is a conceptual architecture for review, not a description of implemented code. The current repository does not contain an application source tree or build configuration.

Related: [Product Vision](../PRODUCT_VISION.md).

## 1. Verified baseline

As of the initial repository inspection:

- The repository contains `README.md`, `LICENSE`, and `.gitignore`.
- The README describes an open-source desktop 3D creation tool, modeling, a future game engine, and possible tools for architecture, engineering, and design.
- Rust-related ignore patterns appear in `.gitignore`, but no `Cargo.toml`, Rust source files, or build configuration are present.
- No UI toolkit, renderer, platform target, scene format, test framework, or CI workflow has been selected in the repository.

Accordingly, the design below defines boundaries and decisions to investigate. It does not commit Usta to a programming language or library.

## 2. Architectural goals

The first architecture should support the candidate 3D Scene Workbench MVP while leaving room for later growth.

- A scene can be edited independently of its on-screen rendering.
- UI events modify document state through explicit commands.
- The viewport presents the scene but is not the only source of truth for it.
- File persistence is versioned and testable without launching the UI.
- The initial application remains simple; abstractions are added only where a real requirement justifies them.
- The technical stack and supported platform are selected and documented before implementation depends on them.

## 3. Proposed logical boundaries

These are conceptual modules. They need not map one-to-one to crates, packages, or processes.

### Desktop shell

Responsibilities:
- application entry point and lifecycle;
- window creation and shutdown;
- routing operating-system input and window events;
- platform-specific integration.

The shell should contain as little scene or rendering business logic as practical.

### Application layer

Responsibilities:
- coordinate user actions as commands and queries;
- connect the UI to document operations;
- report errors in a form the UI can present;
- manage the current document and unsaved-change state.

This layer should make key user workflows testable without simulating every UI interaction.

### Scene/document core

Responsibilities:
- scene and object identity;
- object hierarchy if/when required;
- transforms and references to geometry/material resources;
- selection-independent document mutations;
- validation of document invariants.

The core should not require a graphics device, window, or UI runtime to run its unit tests. Avoid putting GPU handles or UI widget state into the persistent scene model.

### Viewport and renderer

Responsibilities:
- render a scene snapshot or renderable representation;
- manage camera and viewport presentation state;
- translate viewport interactions into application commands;
- handle graphics-device resources and frame lifecycle.

The first implementation may use a single renderer backend. A backend abstraction should only be introduced if there is a clear need; however, renderer-specific types should not leak unnecessarily into the scene/document core.

### Project persistence and asset access

Responsibilities:
- serialize and deserialize the document;
- validate file version and required fields;
- manage references to geometry and material assets;
- provide clear errors for corrupt, unsupported, or missing data.

The project format and whether resources are embedded or stored beside the project are open decisions. Use a schema version from the first persisted format so that future migrations can be designed intentionally.

### UI/presentation

Responsibilities:
- menus, panels, object hierarchy, property editing, and status/error messages as they are introduced;
- display transient UI state such as selection, active tool, and panel layout;
- dispatch user intent to the application layer.

Transient presentation state should be distinguished from durable scene state. Decide explicitly which editor preferences, if any, are persisted.

## 4. Proposed dependency direction

A useful initial dependency direction is:

```text
Desktop Shell
    |
    v
Application / Commands
    |                 \
    v                  v
Scene/Document Core   UI Presentation
    |
    v
Document Operations / Validation

Viewport <---- scene snapshot / renderable view ---- Scene Core
Persistence <---- document data ----> Scene Core
Platform adapters ----> shell, input, filesystem, graphics setup
```

This diagram describes responsibilities, not a finalized package graph. In particular, the exact renderer-to-scene data boundary should be validated with a small technical spike.

## 5. Initial domain model proposal

Keep the first domain model small:

- **Scene document:** one editable scene with a schema version.
- **Object/entity:** stable identifier plus the data needed to represent an item in the scene.
- **Transform:** position, rotation, and scale using a documented coordinate-system convention.
- **Geometry reference:** points to a built-in primitive or a later asset resource.
- **Material reference:** optional in the earliest proof; extend after a minimal rendering path is working.

An entity-component system, complex scene graph, node graph, plugin framework, and full asset database are not prerequisites for the first vertical slice. Select data structures from demonstrated needs, not from the ambition of the eventual product.

## 6. Candidate first vertical slice

Build and verify one end-to-end path:

1. Start the application and open a viewport.
2. Create a new in-memory scene.
3. Render a ground grid and one primitive.
4. Move the camera and select the primitive.
5. Change the primitive's transform via a document command.
6. Save the scene to a versioned local file.
7. Load the file and verify the expected document state.

The first proof should validate that the chosen desktop shell, UI toolkit, and graphics backend can work together. It should not be mistaken for a complete editor.

## 7. Testing and quality gates

### Unit tests
- transform and document operations;
- document invariant validation;
- serialization/deserialization and schema-version handling;
- malformed and unsupported project-file behavior.

### Integration/smoke checks
- clean checkout builds with documented commands;
- application launches on the declared target platform;
- the initial viewport renders;
- the primary create/edit/save/reopen path works.

### Repository checks
- code formatting and static checks for the selected language;
- CI executes the agreed checks on each pull request;
- release and support claims are backed by recorded results.

No test or build should be reported as passing until it has actually been run and its result captured.

## 8. Security and reliability considerations

- Treat imported assets and project files as untrusted input.
- Validate data before accepting it into the document model.
- Avoid executing embedded scripts or arbitrary code when opening a project unless such a feature is explicitly designed and secured in a future phase.
- Do not silently overwrite or discard a document when loading fails.
- Define backup/recovery behavior before the editor is relied on for important work.

These are design considerations; no implementation currently exists to verify them.

## 9. Open architecture decisions

The following decisions should be recorded as ADRs after small, evidence-based evaluations:

1. **Language and build system:** Rust is one possibility suggested by `.gitignore`, not a confirmed choice.
2. **Desktop UI and windowing:** compare candidate toolkits against target OS support, licensing, accessibility, packaging, and integration with the chosen rendering path.
3. **Graphics API and renderer:** compare setup complexity, target hardware, debugging tools, and long-term maintenance.
4. **Scene/document representation:** choose straightforward data structures that support stable identity, transformations, and validation.
5. **Project file and asset layout:** choose a versioned format and define missing-resource and migration behavior.
6. **Initial platform and CI:** define the first supported OS and a reproducible build/test environment.

For each ADR, record context, alternatives, evaluation evidence, decision, consequences, and date. Do not infer a decision merely from an ignore file or an untested prototype.

## 10. Recommended next engineering step

Run a time-boxed **technology spike** before building broad functionality. It should:
- identify the first target operating system;
- evaluate at least two plausible UI/rendering combinations against explicit requirements;
- produce a minimal window with a working 3D viewport and basic input;
- document build commands and observed limitations;
- record the chosen stack in an ADR only after the spike succeeds.

After that, scaffold the production project structure, establish CI, and implement the scene-create/save/reopen vertical slice. Avoid creating speculative engine subsystems until the first workflow has been proven.

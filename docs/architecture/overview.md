# Usta Architecture Overview

> **Status: Draft proposal — not yet approved.** This document describes the intended boundaries and the subset currently demonstrated by the prototype. It is not a claim that Usta has a production-ready editor architecture.

Related: [Product Vision](../PRODUCT_VISION.md).

## 1. Verified baseline

The initial repository was a README/license/gitignore scaffold. The current `spike/bevy-scene-workbench` branch adds:

- a Cargo project with Bevy 0.20.0 pinned for the prototype, plus Serde and Serde JSON;
- `src/main.rs`, which implements a cube viewport, camera interaction, selection, keyboard transforms, and a minimal inspector/outliner presentation;
- `src/document.rs`, which implements a renderer-independent schema-v1 scene document with stable object IDs and translation/rotation/scale persistence;
- Ctrl+S/Ctrl+O JSON save/load through the fixed working-directory path `usta_scene.json`;
- unit tests for document validation/round-trip and camera/transform operations;
- a GitHub Actions workflow for formatting, all-target compile checks, and tests on Ubuntu Linux.

This prototype remains limited: only cube primitives are supported, the outliner is a simple text presentation, there is no file picker or undo/redo, and camera/light/editor UI state is not serialized. The current code has not yet been manually exercised on a desktop OS. Bevy is a prototype baseline, not a final architecture decision.

## 2. Architectural goals

The first architecture should support the candidate 3D Scene Workbench MVP while leaving room for later growth.

- A scene can be edited independently of its on-screen rendering.
- UI events modify document state through explicit commands.
- The viewport presents the scene but is not the only source of truth for it.
- File persistence is versioned and testable without launching the UI.
- The initial application remains simple; abstractions are added only where a real requirement justifies them.
- The prototype stack is documented, while the final UI/rendering choice and supported desktop platform remain open and evidence-driven.

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

The current `SceneDocument` / `SceneObject` types live in `src/document.rs` and contain no Bevy entity IDs, GPU handles, or widget state. They can be parsed, validated, and round-tripped without starting the renderer. The current schema stores the object ID, name, cube primitive kind, translation, quaternion rotation in [x, y, z, w] order, and scale.

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

The prototype currently uses JSON schema version 1 for editable cube objects, with a limit of 256 objects and validation for unsupported versions, duplicate/empty IDs, non-finite values, invalid quaternions, and near-zero scale. Geometry assets are not externalized yet. File picker UX, broader asset layout, and future schema migrations remain open decisions.

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

## 5. Prototype domain model and future extensions

Keep the first domain model small:

- **Scene document:** one editable scene with a schema version.
- **Object/entity:** stable identifier plus the data needed to represent an item in the scene.
- **Transform:** position, rotation, and scale using a documented coordinate-system convention.
- **Geometry reference:** points to a built-in primitive or a later asset resource.
- **Material reference:** optional in the earliest proof; extend after a minimal rendering path is working.

The current implementation intentionally avoids making Bevy's ECS entities the durable document identity. A complex scene graph, node graph, plugin framework, and full asset database are not prerequisites for the first vertical slice. Select data structures from demonstrated needs, not from the ambition of the eventual product.

## 6. Current vertical slice and remaining verification

The current prototype contains the following path in code:

1. Start a desktop window and render a grid, ground plane, light, and cube.
2. Orbit/pan/zoom the camera.
3. Select the cube and edit its transform via keyboard commands.
4. Save the editable objects to schema-v1 JSON with Ctrl+S.
5. Load/validate the file with Ctrl+O, replacing the live objects only after full parsing and validation succeeds.

The CI compile check has passed for the current source commit, but the test stage is still running at the time this document is updated. A manual desktop run is also outstanding. The outliner remains simple text, and there is no file picker, editable numeric property fields, or undo/redo. These limits mean the prototype is not yet a complete editor.

## 7. Testing and quality gates

### Unit tests currently present
- camera orbit invariants and transform commands;
- JSON/file round-trip of stable object identity and transforms;
- unsupported schema version and malformed JSON rejection;
- duplicate IDs, empty scenes, non-finite transform values, zero-length quaternions, and overflowing quaternion magnitude rejection.

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

The document layer validates project data before applying a load and writes through a temporary file before replacement. This does not yet cover imported assets, backup/recovery, or a manual end-to-end runtime test; those remain future reliability work.

## 9. Open architecture decisions

The following decisions should be recorded as ADRs after small, evidence-based evaluations:

1. **Long-term UI/windowing:** Bevy UI is used for prototype overlays, but a production editor may need more capable panels, docking, and accessibility.
2. **Long-term rendering:** Bevy's renderer is the current spike baseline; compare against egui + WGPU before committing to a general-purpose editor architecture.
3. **Supported desktop platform:** CI currently checks Ubuntu Linux; the product's first supported OS and packaging targets are not formally declared.
4. **Scene/document evolution:** schema v1 and stable string IDs exist; object hierarchy, additional primitive/resource kinds, migration policy, and compatibility guarantees remain open.
5. **File UX:** the current filename is fixed to `usta_scene.json`; file picker, project paths, dirty state, and backup/recovery behavior remain future decisions.

For each ADR, record context, alternatives, evaluation evidence, decision, consequences, and date. Do not infer a decision merely from an ignore file or an untested prototype.

## 10. Recommended next engineering step

Finish verifying the existing vertical slice before broadening the application:
- complete the current CI test run and record the result;
- manually run the desktop app and verify selection, camera controls, Ctrl+S, Ctrl+O, and preservation of the current scene after a failed load;
- compare a small egui + WGPU editor-panel experiment with the current Bevy approach before making the long-term stack choice;
- then implement a configurable file path/file picker, unsaved-change state, and a genuine hierarchy/inspector.

Avoid building speculative engine subsystems until the first end-to-end workflow has been proven on a declared target platform.

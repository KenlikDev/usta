# Usta

Usta is an open-source desktop 3D creation tool focused on modeling, with a possible future path toward a game engine and tools for architects, engineers, and designers.

> **Current status:** experimental foundation / technology spike. The current branch includes a Bevy-based viewport prototype; it is not yet a usable 3D editor and does not include persistent scene files or real object-editing tools.

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

- **Right mouse button + drag:** orbit the camera.
- **Middle mouse button + drag:** pan the view.
- **Mouse wheel:** zoom.
- **Home:** reset the camera.

The prototype contains a cube, ground plane, grid, light, and informational UI overlays. The outliner/inspector panel is static and does not currently select or edit objects.

## Quality checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

These commands are configured in GitHub Actions. Do not treat a passing build as proof of runtime rendering; the prototype must also be launched and manually checked on the declared target OS.

## License

Usta is distributed under the [MIT License](LICENSE).

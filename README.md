# Backbone Module Skeleton

A minimal, copy-ready starting point for new Backbone Framework modules.
It ships with exactly **one** reference entity (`Example`) wired end-to-end
so you can rename it to your own domain concept and start generating.

## What you get

- A single schema model at `schema/models/example.model.yaml`
- No migrations yet: `metaphor schema generate` writes them from the schema
- A complete DDD layer cake for `Example`:
  - Domain entity + repository trait
  - Application service (type alias over `GenericCrudService`)
  - Application DTOs (Create / Update / Patch / Response)
  - Infrastructure repository (thin newtype over `GenericCrudRepository`)
  - Presentation HTTP handler
  - Routes
  - Seeder
- A `Module` struct wiring the service into the framework

## Directory layout

The tree below shows the **complete canonical Backbone module structure**.
This skeleton ships only the minimum viable subset (one `Example` entity and
the core DDD layers); every other folder is documented here
so you know where to add the optional layers when you need them.

```
backbone-module/
│
├── schema/                              # SCHEMA DEFINITIONS — Single Source of Truth
│   ├── models/                          # Entity schema definitions
│   │   └── example.model.yaml           # The one reference entity (rename me)
│   ├── hooks/                           # Lifecycle hooks and triggers
│   ├── workflows/                       # Business workflow definitions
│   └── openapi/                         # OpenAPI / Swagger specifications
│
├── migrations/                          # DATABASE MIGRATIONS (PostgreSQL), written by
│   │                                    #   `metaphor schema generate`; none ship here
│   ├── 001_create_enums.up.sql          # Enum types (e.g. example_status)
│   ├── 001_create_enums.down.sql
│   ├── 002_create_example_table.up.sql  # CREATE TABLE for the example entity
│   └── 002_create_example_table.down.sql
│
├── src/                                 # SOURCE CODE (generated + custom)
│   │
│   ├── lib.rs                           # Module entry point + re-exports
│   ├── module.rs                        # `Module` struct — wires service into framework
│   │
│   ├── domain/                          # Domain Layer — pure business model
│   │   ├── entity/                      # Entity structs + trait impls
│   │   │   └── example.rs
│   │   ├── repositories/                # Repository traits (ports)
│   │   │   └── example_repository.rs
│   │   ├── value_objects/               # Value objects
│   │   ├── event/                       # Domain events
│   │   ├── state_machine/               # State transition definitions
│   │   ├── services/                    # Domain services
│   │   ├── specifications/              # Specification pattern
│   │   └── permission/                  # Permission rules
│   │
│   ├── application/                     # Application Layer — use cases & orchestration
│   │   ├── dto/                         # Create / Update / Patch / Response DTOs
│   │   │   └── example_dto.rs
│   │   ├── service/                     # Application services
│   │   │   ├── example_service.rs       # Type alias over GenericCrudService
│   │   │   └── error.rs                 # Service-level error types
│   │   ├── usecases/                    # Use case implementations
│   │   ├── commands/                    # CQRS commands
│   │   ├── queries/                     # CQRS queries
│   │   ├── validator/                   # Input validation
│   │   ├── workflows/                   # Workflow orchestration
│   │   ├── triggers/                    # Database trigger handlers
│   │   ├── bulk_operations/             # Bulk import/export
│   │   ├── auth/                        # Module-specific auth
│   │   ├── middleware/                  # Application middleware
│   │   └── subscriptions/               # Event subscriptions
│   │
│   ├── infrastructure/                  # Infrastructure Layer — adapters
│   │   ├── persistence/                 # Repository implementations
│   │   │   └── example_repository_impl.rs   # Postgres repo via GenericCrudRepository
│   │   ├── event_store/                 # Event sourcing storage
│   │   ├── projections/                 # CQRS read-model projections
│   │   ├── cache/                       # Caching adapters
│   │   ├── rate_limiter/                # Rate limiting
│   │   ├── jobs/                        # Background jobs
│   │   ├── messaging/                   # Message bus adapters
│   │   ├── external/                    # Third-party integrations
│   │   ├── metrics/                     # Prometheus metrics
│   │   └── health/                      # Health check endpoints
│   │
│   ├── presentation/                    # Presentation Layer — transport
│   │   ├── http/                        # REST / Axum handlers
│   │   │   └── example_handler.rs       # BackboneCrudHandler wiring
│   │   ├── grpc/                        # gRPC services
│   │   ├── graphql/                     # GraphQL resolvers
│   │   ├── cli/                         # CLI subcommands
│   │   ├── dto/                         # Wire-format DTOs
│   │   ├── middleware/                  # Transport middleware
│   │   └── versioning/                  # API versioning
│   │
│   ├── routes/                          # Route composition
│   │   └── example_routes.rs
│   │
│   ├── seeders/                         # Sample data for `backbone seed run`
│   │   └── example_seeder.rs
│   │
│   ├── handlers/                        # Custom handler entry points
│   ├── integration/                     # Inter-module integration adapters
│   └── exports/                         # Public API exports
│
├── proto/                               # PROTOBUF DEFINITIONS (generated from schema)
│   ├── domain/
│   │   └── entity/                      # Entity messages
│   └── services/                        # Service definitions
│
├── tests/
│   ├── integration_tests.rs             # Stub — replace with your own test suite
│   └── integration/                     # Integration test fixtures
│
├── config/                              # MODULE CONFIGURATION
│   ├── application.yml                  # Default runtime config (db, server, log)
│   ├── application-dev.yml              # Development overrides
│   └── application-prod.yml             # Production overrides
│
├── docs/                                # Module-specific documentation
├── benches/                             # Criterion benchmarks
│
├── buf.yaml                             # Protobuf lint config
├── Cargo.toml                           # Trimmed deps — update `path = "..."` after copying
└── README.md                            # This file
```

> **What ships in this skeleton:** `schema/models/example.model.yaml`,
> `Cargo.toml`, `README.md`, `buf.yaml`, `config/application.yml`,
> `tests/integration_tests.rs`, and the `src/` layers `domain/{entity,repositories}`,
> `application/{dto,service}`, `infrastructure/persistence`, `presentation/http`,
> `routes`, `seeders`, plus `lib.rs` and `module.rs`.
> Everything else in the tree above is a **placeholder for layers you can add later**.

## Getting started

1. **Create** the module: `metaphor module create <name>`. It is published as
   `backbone-<name>` (a bare name gets the prefix), so the crate, its directory and
   its repository are all `backbone-<name>`. The command also starts it at
   `0.1.0`, points `repository` at `faridlab/backbone-<name>`, and makes it
   publishable. The `backbone-*` dependencies come from crates.io by semver
   requirement, the same as every other module.
2. **Rename** `example` to your entity name throughout:
   - `schema/models/example.model.yaml` → `<your_entity>.model.yaml`
   - Inside the YAML, change `Example`, `examples`, `ExampleStatus`
   - The matching `src/` files
3. **Generate** the code and the migrations from the schema:

   ```bash
   metaphor schema generate backbone-<name> --target all --force
   ```

4. **Run migrations**:

   ```bash
   DATABASE_URL="postgresql://..." metaphor migration run
   ```

## Custom code (regeneration safety)

Anywhere you see a `// <<< CUSTOM` / `// END CUSTOM` marker, the content in
between is preserved across regeneration. For code outside those markers, use
the `_custom` suffix convention:

- `order_photo_service_custom.rs` — never rewritten
- Register in `mod.rs` beneath a `// <<< CUSTOM` marker
- Wire custom HTTP endpoints via `custom_routes.rs`, not the generated handler

## Going further

This skeleton intentionally excludes the optional layers (event store, cache,
gRPC, GraphQL, CLI, triggers, validators, workflows, state machines, ...).
Add them back from the full framework docs as you need them. The directory
structure mirrors what the generator expects, so adding a new layer is as
simple as creating the corresponding `mod.rs` and pointing `lib.rs` at it.
## Releasing

Modules are published to crates.io and consumed by semver version, never by
git tag. To release, bump `version` in `Cargo.toml` (below 1.0, a fix or an
additive change is a patch bump and a breaking change is a minor bump) and merge
it to `main`. The [release workflow](.github/workflows/crate-release.yml) calls the
shared backbone crate release workflow: every pull request is built, tested,
audited (the package must still carry `migrations/` and `schema/`, and no
secrets or dotfiles) and dry-run published; a push to `main` whose version is
not on crates.io yet publishes it and then pushes the `v<version>` tag. Never push
a release tag by hand. It needs the `CARGO_REGISTRY_TOKEN` repository secret.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT), at your option.

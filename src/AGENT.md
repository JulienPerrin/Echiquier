# JavaScript/TypeScript and HTML Best Practices for the Echiquier Project

## 1. Project Structure & File Organisation

- **src/** – All source files live here.
- Sub‑folders by concern:
  - **ui/** – UI components and helpers.
  - **api/** – API/tauri wrappers.
  - **state/** – Reactive signals.
  - **utils/** – Pure helper functions and type helpers.
- `main.ts` is only an entry point that imports `app.ts`; it contains no logic.
- Keep files small (≤ 200 lines when possible) so they can be read and reasoned about quickly.

## 2. TypeScript Configuration

- `tsconfig.json` uses **ESNext** modules suitable for Vite.
- `noUnusedLocals`/`noUnusedParameters` is enabled – always remove dead code.
- `strict` is `true` – never allow implicit `any`.
- Prefer **interfaces** over type aliases for public contracts because they are extendable.
- Use discriminated unions for state representation where a value can be one of several variants.

## 3. Using @preact/signals-core

- Signals are plain `signal<T>()` values exported from `state.ts`.
- Computed values (`computed(() => ...)`) are used for derived state (like `gameTitle`).
- `effect(() => ...)` automatically updates the DOM when the signal it accesses changes.
- Avoid mutating signal values directly – always use `signal.value = newValue` for a state update.

## 4. UI Component Pattern

- Each UI piece has a dedicated module (e.g., `headers.ts`, `moves.ts`).
- Modules export `render…` and `update…` helpers that are pure functions operating on the DOM.
- They keep DOM queries scoped locally and never assume global document state.
- The main logic keeps the app in `app.ts` where `initApp` hooks events, sets initial state, and starts the UI.

## 5. API / Tauri Interaction

- `api.ts` centralises all `invoke()` calls. It exposes whether the runtime is Tauri via `isTauri`.
- All HTTP or Tauri calls are async functions returning typed results.
- A tiny testable shim is available for the browser: fallback to `fetch()`.

## 6. Clipboard & Toast

- Clipboard helpers live in `share.ts` and use `navigator.clipboard` or Tauri API.
- Global toast notifications are controlled through small helper functions in `toast.ts`.
- No side effects outside their own module – easy to test.

## 7. Testing (Vitest)

- Tests live under `src/*` with a matching `.spec.ts` suffix.
- No `@ts-migrate` in play – all tests run with TypeScript‑checked `vitest`.
- Coverage is enabled automatically via `npm run coverage`.
- To run a single file: `vitest run -f src/utils.spec.ts`.

## 8. HTML Template (`index.html`)

- Dynamic values (`%TITLE%`, `%GAME_OPTIONS%`) are replaced by a pre‑processor.
- All UI references use stable `id`s – queries are fast and scoped.
- Accessibility attributes (ARIA, `role`, `aria‑live`, `tabindex`) are used throughout.
- Styles are in `src/style.css`, imported once at the root.

## 9. General Coding Guidelines

- **No `any`.** Use `unknown` or proper union types.
- **Prefer `const` over `let`** when a value is never reassigned.
- **Always document public helper functions** with JSDoc or TypeScript comment blocks.
- **Keep side‑effects isolated** – unit tests should import only pure functions.
- **Use consistent import paths** (relative imports, no aliasing unless absolutely necessary).
- **Linting & formatting** – run `npm run lint` or set up a pre‑commit hook.

## 10. Testing & Coverage

- **Run tests**: `npm test` (executes `vitest run`).
- **Generate coverage report**: `npm run coverage` (executes `vitest run --coverage`).
  - Reports are generated under the `coverage` directory, providing a JSON summary and a human‑readable HTML page (`coverage/index.html`).
  - The coverage script uses Istanbul's v8 reporter, supporting modern JavaScript syntax.
- **Coverage thresholds**: Not enforced by default, but you can add a `coverageThreshold` section to `vitest.config.ts` if you want to enforce minimum percentages.
- **Testing UI rendering**: Tests use JSDOM (the default Vite test environment) so all DOM queries (`document.querySelector`) work as expected. Keep mocked DOM elements in tests when the actual application elements are absent.
- **Mocking external APIs**: In tests, use `vi.fn()` to mock `fetch` or `invoke` calls from `api.ts`. This keeps tests deterministic and avoids network or Tauri dependencies.
- **Test naming**: Follow the pattern `describe('module', ...)->it('should do X', ...)` for clarity.
- **Coverage analysis**: After running coverage, open `coverage/index.html` to visually see uncovered branches and lines. Refactor or add tests to hit missing branches.
- **CI integration**: Add a CI job that runs `npm test` and `npm run coverage`; fail if coverage drops below a threshold.

---

These guidelines keep the codebase maintainable, readable, and testable while preserving the fast dev‑experience that Vite and Preact provide.

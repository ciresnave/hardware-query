# Roadmap / open questions

## Open product questions

- **Is "AI-ready by memory alone" intended?** `SystemOverview::is_ai_ready()`
  is an OR of three independent signals: `cpu.ai_capable`, `gpu.ai_capable`,
  `memory_gb >= 8.0`. `SystemOverview::ai_score()` caps memory's own
  contribution at 25/100 (even at >=32GB), so a system judged "ready" purely
  via the memory branch (no AI-capable CPU or GPU) can score as low as 15 --
  mathematically unreachable above ~25 through that branch alone. Surfaced
  by `tests/simplified_api_tests.rs::test_ai_readiness_assessment` failing
  on a real `macos-latest` CI runner (0.2.3, aarch64, no AI-capable CPU
  features detected, readiness triggered by memory alone) -- see
  `CHANGELOG.md`'s 0.2.3 entry. The test's floor was lowered to 15 (the
  true achievable minimum) to unblock CI, with no change to `is_ai_ready()`
  or `ai_score()`'s weights.

  Decide: is memory-alone readiness at a 15/100 score the intended product
  behavior, or should the memory branch's weight increase, or should
  `is_ai_ready()` require memory plus at least one other signal? No action
  taken pending this decision.

- **`src/arm.rs`'s Raspberry Pi / NVIDIA Jetson detection is gated by
  `target_arch` only, not `target_os`.** `detect_raspberry_pi()` reads
  `/proc/device-tree/model` and `/proc/cpuinfo`, and `get_memory_size()`
  reads `/proc/meminfo` -- all Linux-only paths -- under
  `#[cfg(target_arch = "aarch64")]` with no `target_os = "linux"` alongside
  it. Surfaced on 0.3.0's CI run: `macos-latest` is Apple Silicon
  (aarch64), the first non-Linux aarch64 runner this crate has ever built
  on, and this code now compiles (and at runtime would uselessly fail to
  find those paths) on aarch64 macOS and aarch64 Windows too. Needs its own
  PR adding `target_os = "linux"` to the gate (or an explicit macOS/Windows
  arm would-be-identical-looking function that actually detects the right
  things there) -- not folded into the clippy fix that found it.

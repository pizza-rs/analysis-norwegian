<div align="center">

# 🇳🇴 pizza-analysis-norwegian

**Norwegian text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--norwegian-blue)](https://github.com/pizza-rs/analysis-norwegian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Norwegian (Bokmål) language analysis with light stemming and stop words.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `norwegian_light_stem` | Norwegian light stemmer |
| TokenFilter | `norwegian_stop` | Norwegian stop words (172 entries) |
| Analyzer | `norwegian` | Full pipeline: lowercase → light_stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_norwegian::register_all(&mut factory);

let analyzer = factory.get_analyzer("norwegian").unwrap();
// "husene" (the houses) → "hus"
```

## Installation

```toml
[dependencies]
pizza-analysis-norwegian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["norwegian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>

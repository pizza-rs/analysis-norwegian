# pizza-analysis-norwegian

Norwegian language analysis (Bokmål and Nynorsk) with light stemmer and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `norwegian_stem` | Token Filter | Norwegian light stemmer — handles noun, adjective, and verb suffixes |
| `norwegian_stop` | Token Filter | Norwegian stop words filter (172 words) |
| `norwegian` | Analyzer | Full pipeline: lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "norwegian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["norwegian_stem", "norwegian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers

# Choose File via Native File Dialog

```sql
SELECT * FROM read_csv(choose_file());
```

https://github.com/user-attachments/assets/2067ff33-5159-44cd-82e3-c117bcbdf9e0

## Option

You can filter files by the extension. For example, in order to show CSV files
only, you can specifies this:

```sql
choose_file('csv')
```

## Requirements

- DuckDB v2.0.0. The extension is built on the C API v2 via [`duckdb-neo`](https://crates.io/crates/duckdb-neo)
  and is pinned to the exact DuckDB version it was built for (`TARGET_DUCKDB_VERSION` in the `Makefile`).

## Installation

file_dialog is available from the Community Extension repository.

```sql
INSTALL file_dialog FROM community;
LOAD file_dialog;
```

## Development

```shell
make configure
make debug
```

```sql
LOAD './build/debug/extension/file_dialog/file_dialog.duckdb_extension';
```

### Updating the DuckDB version

Change these together:

- `Cargo.toml`: `duckdb-neo` and `duckdb-loadable-macros` versions (`0.2MMPP.x` / `1.2MMPP.x` encode DuckDB `2.MM.PP`)
- `Makefile`: `TARGET_DUCKDB_VERSION`
- `.github/workflows/MainDistributionPipeline.yml`: `duckdb_version`, `ci_tools_version` and the workflow ref
- `src/lib.rs`: `api_version` in `#[duckdb_entrypoint_c_api_v2(...)]`
- the `extension-ci-tools` submodule

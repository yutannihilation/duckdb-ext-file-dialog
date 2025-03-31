# Choose File via Native File Dialog

```sql
SELECT * FROM read_csv(choose_file());
```

## Development

```shell
make configure
make debug
```

```sql
LOAD './build/debug/extension/file_dialog/file_dialog.duckdb_extension';
```
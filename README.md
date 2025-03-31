# Choose File via Native File Dialog

```sql
SELECT * FROM read_csv(choose_file());
```

![](./file-dialog2.mp4)

## Development

```shell
make configure
make debug
```

```sql
LOAD './build/debug/extension/file_dialog/file_dialog.duckdb_extension';
```
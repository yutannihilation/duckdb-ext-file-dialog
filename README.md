# Choose File via Native File Dialog

```sql
SELECT * FROM read_csv(choose_file());
```

https://github.com/user-attachments/assets/2067ff33-5159-44cd-82e3-c117bcbdf9e0

## Development

```shell
make configure
make debug
```

```sql
LOAD './build/debug/extension/file_dialog/file_dialog.duckdb_extension';
```

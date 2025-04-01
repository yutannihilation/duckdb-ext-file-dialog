# Choose File via Native File Dialog

```sql
SELECT * FROM read_csv(choose_file());
```

![](./file-dialog2.mp4)

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
### DB
```bash
echo DATABASE_URL=/path/to/your/sqlite/database.db > .env
diesel setup
diesel migration generate --diff-schema initial
diesel migration run
diesel migration redo
```

### App
```bash
dx build --platform web
dx serve --platform web
```
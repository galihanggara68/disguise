# Python Environments with Disguise

Manage virtual environments and dependency locking.

## Scripts Configuration

```bash
# Create and update virtualenv
disguise add --name "py-init" \
             --command "python3 -m venv .venv && source .venv/bin/activate && pip install -r requirements.txt" \
             --description "Initialize fresh virtualenv and install deps" \
             --tags "python,env"

# Export pinned requirements
disguise add --name "py-freeze" \
             --command "pip freeze > requirements.txt" \
             --description "Snapshot current dependencies" \
             --tags "python,deps"

# Run a python script with a module
disguise add --name "py-run" \
             --command "python3 -m" \
             --description "Run a python module" \
             --tags "python,run"
```

## Usage Examples

### Initializing a project
```bash
disguise run py-init
```

### Running a module like flask
```bash
disguise run py-run -- flask run --port 5001
```

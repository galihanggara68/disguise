# Node.js Development with Disguise

Streamline package management and build processes.

## Scripts Configuration

```bash
# Clean install
disguise add --name "node-refresh" \
             --command "rm -rf node_modules package-lock.json && npm install" \
             --description "Nuke node_modules and reinstall everything" \
             --tags "node,npm"

# Run tests with coverage
disguise add --name "node-test" \
             --command "npm test -- --coverage" \
             --description "Run vitest/jest with coverage report" \
             --tags "node,test"

# Run a specific script from package.json in background
disguise add --name "node-dev" \
             --command "npm run dev" \
             --description "Start development server" \
             --tags "node,dev"
```

## Usage Examples

### Starting dev server in background
```bash
disguise run node-dev --background
```

### Running tests for a specific file
```bash
disguise run node-test -- src/auth/login.test.ts
```

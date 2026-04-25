# Swiss Army Knife Utilities with Disguise

A collection of general-purpose utility scripts that are useful across any directory on your system. These scripts leverage Disguise to keep your "global toolbox" organized.

## Scripts Configuration

```bash
# Find the top 10 largest files in the current directory
disguise add --name "top-files" \
             --command "find . -type f -exec du -h {} + | sort -rh | head -n 10" \
             --description "Identify the largest files in the current tree" \
             --tags "utils,disk"

# Check what process is using a specific port
disguise add --name "port-who" \
             --command "lsof -i :" \
             --description "Find the process listening on a port" \
             --tags "network,debug"

# Spin up a quick HTTP server for file sharing
disguise add --name "serve" \
             --command "python3 -m http.server" \
             --description "Start a static file server in the current dir" \
             --tags "network,web"

# Get your local IP address
disguise add --name "my-ip" \
             --command "hostname -I | awk '{print \$1}'" \
             --description "Display local network IP address" \
             --tags "network,utils"

# Cleanup common temporary files and caches
disguise add --name "clean-junk" \
             --command "find . -name '.DS_Store' -type f -delete && find . -name '__pycache__' -type d -exec rm -rf {} +" \
             --description "Remove OS and language junk files recursively" \
             --tags "maintenance,utils"
```

## Usage Examples

### Finding large files
```bash
disguise run top-files
```

### Checking a specific port
```bash
disguise run port-who -- 8080
```

### Serving files to your local network
Run this in the background so it doesn't block your terminal:
```bash
disguise run serve -- 9000 --background
```
*Your server logs will be captured at `~/.config/disguise/logs/serve.log`.*

# Docker Workflow with Disguise

Manage your Docker environment more efficiently by grouping common cleanup and maintenance tasks.

## Scripts Configuration

Add these to your `disguise` setup:

```bash
# Cleanup all unused containers, networks, and images
disguise add --name "docker-purge" \
             --command "docker system prune -a --volumes -f" \
             --description "Deep clean Docker environment" \
             --tags "docker,maintenance"

# Stop all running containers
disguise add --name "docker-stop-all" \
             --command "docker stop \$(docker ps -q)" \
             --description "Stop all active containers" \
             --tags "docker,utils"

# Tail logs of a specific container with dynamic name
disguise add --name "docker-logs" \
             --command "docker logs -f" \
             --description "Tail container logs" \
             --tags "docker,debug"
```

## Usage Examples

### Running a deep clean
```bash
disguise run docker-purge
```

### Debugging a specific container
Pass the container name as a dynamic argument:
```bash
disguise run docker-logs -- my-nginx-container
```

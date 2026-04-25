# Kubernetes Management with Disguise

Avoid long `kubectl` commands by aliasing common operations.

## Scripts Configuration

```bash
# Switch namespace quickly
disguise add --name "k-ns" \
             --command "kubectl config set-context --current --namespace" \
             --description "Change current kubectl namespace" \
             --tags "k8s,config"

# Get all resources in current namespace
disguise add --name "k-get-all" \
             --command "kubectl get all" \
             --description "List all pods, services, and deployments" \
             --tags "k8s,view"

# Port forward a service
disguise add --name "k-forward" \
             --command "kubectl port-forward svc/" \
             --description "Forward a service to local port" \
             --tags "k8s,network"
```

## Usage Examples

### Changing namespace
```bash
disguise run k-ns -- production
```

### Forwarding a service
```bash
disguise run k-forward -- my-api 8080:80
```

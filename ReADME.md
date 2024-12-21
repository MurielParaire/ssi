# Kubernetes security


# Prerequisites

To execute and test this project, you will need a kubernetes cluster running.
It has been tested with a k3d cluster running locally.

# Requirements

A web application will be deployed. It contains:  
- one service returning a random verb (./verbs)  
- one service returning a random noun (./nouns)  
- a web application calling both of these services and displaying the random verb and noun (./aggregator)  
Only the web application is exposed.

To start the application you will need to apply the `kustomization.yaml` file:
```
kubectl apply -k ./infra
```


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

This will deploy everything in the `ssi` namespace.
To test that everything is working, you can connect to the web application. For that, you will need to get the external API of the `agg` service and use the port 8080. Normally, you should see a simple text message as a response with a verb and a noun.

# Kyverno

Install kyverno using helm:
```
helm repo add kyverno https://kyverno.github.io/kyverno
helm repo update
helm install kyverno kyverno/kyverno -n kyverno --create-namespace
```

## admission policies

**Disallow default namespace**
No pods are allowed to be deployed in the default namespace.
You can test it using the file ./infra/kyverno/test/test-default-namespace.yaml.

**Verify image registry**
Only images from ghcr.io/murielparaire/ssi/ are allowed to be loaded.
You can test it using the file ./infra/kyverno/test/test-image-registry.yaml.

**Verify liveness probe**
All pods in the ssi namespace need to have a liveness probe.
You can test it using the file ./infra/kyverno/test/test-liveness.yaml.

You should get an error for each of these files.

## mutation policies

**Add default ressource limit**

This mutation policy adds a default ressource limit for each pod that doesn't have any.
You can check the descriptions of the aggregator and verb pods - the aggregator has a resource limit specified but the verbs do not, but kyverno will add them to it.
You should see a ressource limit of a memory of 100 for the aggregator and 64Mi for the verbs pod.


**Add default security context**

This mutation policy adds a default security context as nonRootUser and default fsGroup etc.
You can check it with these commands:
```
kubectl get pods -n ssi
kubectl exec <podname> -n ssi -- id
```
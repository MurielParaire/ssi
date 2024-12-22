# Kubernetes security


# Prerequisites

To execute and test this project, you will need a kubernetes cluster running.
It has been tested with a k3d cluster running locally as well as have helm installed on a newly created cluster:
```
k3d cluster create <CLUSTER_NAME>
```

# Installation of every tool

## Istio

[Install Istio](https://istio.io/latest/docs/setup/install/helm/) using helm:
```
helm repo add istio https://istio-release.storage.googleapis.com/charts
helm repo update
helm install istio-base istio/base -n istio-system --set defaultRevision=default --create-namespace
helm install istiod istio/istiod -n istio-system --wait
```

Install the necessary CRDs to add a Gateway API:
```
kubectl get crd gateways.gateway.networking.k8s.io &> /dev/null || \
{ kubectl kustomize "github.com/kubernetes-sigs/gateway-api/config/crd?ref=v1.2.0" | kubectl apply -f -; }
```

For the gateway api to work, you will need to remove traefik (if you have it):
```
kubectl -n kube-system delete deployment traefik
kubectl -n kube-system delete svc traefik
```

## Kyverno

[Install kyverno](https://kyverno.io/docs/installation/methods/) using helm:
```
helm repo add kyverno https://kyverno.github.io/kyverno
helm repo update
helm install kyverno kyverno/kyverno -n kyverno --create-namespace
```

## Falco

[Install Falco](https://falco.org/blog/extend-falco-outputs-with-falcosidekick/)
```
helm repo add falcosecurity https://falcosecurity.github.io/charts
helm repo update
helm install --replace falco --namespace falco --create-namespace --set tty=true falcosecurity/falco --set driver.kind=modern_ebpf --set falcosidekick.enabled=true --set falcosidekick.webui.enabled=true
```

# Application

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

To access the application, you need to retrieve its ip:
```
kubectl get svc -n istio-system istio-gateway-istio
```

You first have to simulate the dns by adding a new line in your /etc/hosts file:
```
<EXTERNAL_IP> app.local
```
and then you can connect to the aggregator at: http://app.local.




# Kyverno


## admission policies

**Disallow default namespace**
No pods are allowed to be deployed in the default namespace.
You can test it using:
```
kubectl apply -f ./infra/kyverno/test/test-default-namespace.yaml
```

**Verify image registry**
Only images from ghcr.io/murielparaire/ssi/ are allowed to be loaded.
You can test it using 
```
kubectl apply -f ./infra/kyverno/test/test-image-registry.yaml
```

**Verify liveness probe**
All pods in the ssi namespace need to have a liveness probe.
You can test it using 
```
kubectl apply -f ./infra/kyverno/test/test-liveness.yaml
```

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

# Falco

**To view the Falco UI:**  

While in development, you can simply forward the port:
```
kubectl port-forward svc/falco-falcosidekick-ui -n falco 2802
```
and view it on http://localhost:2302

Please note that the default login and password for falco are `admin` and `admin`.


# Istio

Install kiali to visualize Istio

```
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.24/samples/addons/kiali.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.24/samples/addons/prometheus.yaml
```

Then you can forward the port to view it on kiali:
```
kubectl port-forward svc/kiali 20001:20001 -n istio-system
```
and connect to the interface at http://localhost:20001.

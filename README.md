# Kubernetes security


# Prerequisites

To execute and test this project, you will need a kubernetes cluster running.
It has been tested with a k3d cluster running locally as well as have helm installed.

# Istio

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

Install kiali to visualize Istio

```
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.24/samples/addons/kiali.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.24/samples/addons/prometheus.yaml
```


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

To access the application, you need to retrieve its ip:
```
kubectl get svc -n ssi agg
```
and then you can connect to the aggregator at: http://<EXTERNAL_IP>:8080.

# Kyverno

[Install kyverno](https://kyverno.io/docs/installation/methods/) using helm:
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

# Falco

[Install Falco](https://falco.org/blog/extend-falco-outputs-with-falcosidekick/)
```
helm repo add falcosecurity https://falcosecurity.github.io/charts
helm repo update
helm install --replace falco --namespace falco --create-namespace --set tty=true falcosecurity/falco --set driver.kind=modern_ebpf --set falcosidekick.enabled=true --set falcosidekick.webui.enabled=true
```

**To view the Falco UI:**  

While in development, you can simply forward the port:
```
kubectl port-forward svc/falco-falcosidekick-ui -n falco 2802
```
and view it on http://localhots/2302

Please note that the default login and password for falco are `admin` and `admin`.

For production use, you can update the file ./infra/falco/ingress.yaml with your hostname. Then, just apply the file with:
```
kubectl apply -f infra/falco/ingress.yaml
```

If you want to simulate the dns on your local machine, simply add a new line in your `/etc/hosts` file:
```
<EXTERNAL_IP> falco.local
```
Instead of falco.local you can put another hostname, but you will need to change it in the ingress.yaml file too.
The external ip can be found by looking at the traefik of our kubernetes system. Please note that this implies that your load balancer is traefik, if you have set up something else, you might need to adapt this command.
```
kubectl get svc -n kube-system traefik
```

Then you should be able to access the Falco Sidekick UI at http://falco.local.
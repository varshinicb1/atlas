---
kind: Package
id: package:docker/kubernetes
name: Docker & Kubernetes Patterns
version: "1.30"
purpose: Document container and orchestration patterns — Dockerfile best practices, multi-stage builds, and Kubernetes pods, deployments, services, probes, and resource management.
problem_solved: Provides a reference for packaging reproducible containers and running resilient workloads on Kubernetes, reducing image bloat, crash loops, OOM kills, and misconfigured networking.
install: |
  ```bash
  brew install docker kubectl kind
  ```
dependencies:
  - package:docker
  - package:kubectl
concepts:
  - name: Cloud Native Computing
    id: concept:domain/cloud-native
    description: The broader cloud-native ecosystem (CNCF) of containers, orchestration, and declarative infrastructure these patterns belong to.
  - name: Dockerfile
    id: concept:docker/dockerfile
    description: A declarative recipe of FROM, COPY, RUN, and CMD instructions that builds an immutable container image layer by layer.
  - name: Multi-stage Build
    id: concept:docker/multistage
    description: Using several FROM stages so build tooling lives in early stages and only the runtime artifact is copied into the final slim image.
  - name: Image Layers & Caching
    id: concept:docker/layers
    description: Each instruction creates a layer cached by the build daemon; ordering COPY/package-install after source changes maximizes cache hits.
  - name: Container
    id: concept:docker/container
    description: A runnable instance of an image with its own filesystem, process namespace, and network; isolated but sharing the host kernel.
  - name: Pod
    id: concept:k8s/pod
    description: The smallest deployable Kubernetes unit — one or more co-scheduled containers sharing network and storage volumes.
  - name: Deployment
    id: concept:k8s/deployment
    description: A controller that declares desired replica count and pod template, managing rolling updates and rollbacks via ReplicaSets.
  - name: Service
    id: concept:k8s/service
    description: A stable virtual IP and DNS name that load-balances traffic to a set of pods selected by labels, decoupling clients from pod churn.
  - name: Liveness & Readiness Probes
    id: concept:k8s/probes
    description: "Health checks: liveness restarts unhealthy containers, readiness gates traffic until the pod can serve (preventing premature load)."
  - name: Resource Requests & Limits
    id: concept:k8s/resources
    description: requests guarantee a minimum of CPU/memory for scheduling; limits cap usage and trigger OOMKill or CPU throttling when exceeded.
  - name: ConfigMap & Secret
    id: concept:k8s/config
    description: Externalized configuration (ConfigMap) and sensitive data (Secret) injected as env vars or mounted files, keeping images portable.
  - name: Ingress
    id: concept:k8s/ingress
    description: An API object describing HTTP/HTTPS routing rules from outside the cluster to Services, often backed by a controller like nginx.
  - name: Namespace
    id: concept:k8s/namespace
    description: A virtual cluster partition for scoping resources, quotas, and RBAC within a physical cluster.
  - name: Horizontal Pod Autoscaler
    id: concept:k8s/hpa
    description: Scales replica count automatically based on CPU, memory, or custom metrics; requires metrics-server to be installed.
  - name: StatefulSet
    id: concept:k8s/statefulset
    description: Like a Deployment but for stateful apps, giving pods stable network identities and persistent storage across reschedules.
apis:
  - name: docker build
    id: api:docker/build
    signature: "docker build -t myapp:1.0 -f Dockerfile ."
    returns: A built image tagged locally.
    description: Builds an image from a Dockerfile; leverages layer cache and supports multi-stage targets via --target.
  - name: docker run
    id: api:docker/run
    signature: "docker run -p 8080:80 --rm myapp:1.0"
    returns: A running container.
    description: Creates and starts a container from an image, mapping ports and removing it on exit with --rm.
  - name: kubectl apply
    id: api:k8s/apply
    signature: "kubectl apply -f deploy.yaml"
    returns: The created/updated resources.
    description: Declaratively creates or updates resources from YAML manifests, idempotent across runs.
  - name: kubectl rollout
    id: api:k8s/rollout
    signature: "kubectl rollout status deploy/myapp / kubectl rollout undo deploy/myapp"
    returns: Rollout status or undo result.
    description: Watches rollout progress and can undo to the previous ReplicaSet revision on failure.
  - name: kubectl port-forward
    id: api:k8s/port-forward
    signature: "kubectl port-forward svc/myapp 8080:80"
    returns: A tunnel from localhost to the resource.
    description: Forwards a local port to a pod or service for debugging without exposing it externally.
  - name: kubectl logs
    id: api:k8s/logs
    signature: "kubectl logs deploy/myapp -f"
    returns: Streamed container logs.
    description: Fetches and follows stdout/stderr from a pod or its controller's pods.
  - name: kubectl exec
    id: api:k8s/exec
    signature: "kubectl exec -it pod/myapp -- sh"
    returns: Command output inside the container.
    description: Runs a command in a running container for debugging and inspection.
examples:
  - id: example:docker/multistage
    language: dockerfile
    description: Multi-stage Dockerfile building a Go binary then copying into distroless.
  - id: example:k8s/deployment
    language: yaml
    description: A Deployment with probes, resources, and a rolling update strategy.
  - id: example:k8s/service
    language: yaml
    description: A ClusterIP Service selecting pods by app label.
  - id: example:k8s/hpa
    language: yaml
    description: HPA scaling on CPU utilization with min/max replicas.
failures:
  - id: failure:docker/latest-tag
    symptom: Non-reproducible deployments; different nodes run different code from the same tag.
    cause: Relying on the mutable :latest tag, which is overwritten on each build.
    fix: Tag images with immutable version/commit SHAs and pin them in manifests; never deploy :latest to production.
  - id: failure:docker/root-user
    symptom: Security scanner flags or container escapes the sandbox; host compromised on breakout.
    cause: Running the container process as UID 0 (root) without a USER directive.
    fix: Add a non-root USER in the Dockerfile and drop capabilities; run as a dedicated uid.
  - id: failure:k8s/crashloopbackoff
    symptom: Pod repeatedly starts then crashes; status CrashLoopBackOff.
    cause: App exiting immediately (missing env/secret, bad config, port already in use, or unhandled panic on boot).
    fix: Inspect kubectl logs and kubectl describe; verify ConfigMaps/Secrets exist, fix startup command, add a readiness probe.
  - id: failure:k8s/oomkilled
    symptom: Container restarts with reason OOMKilled and exit code 137.
    cause: Memory usage exceeded the configured limit; the kernel kills the process to protect the node.
    fix: Raise the memory limit/request after profiling, fix leaks, or set a realistic request so the scheduler places it correctly.
  - id: failure:k8s/imagepullbackoff
    symptom: Pod stuck with ErrImagePull / ImagePullBackOff.
    cause: Wrong image name/tag, missing registry credentials (ImagePullSecret), or registry unreachable.
    fix: Verify the image reference, create an ImagePullSecret for private registries, and check registry connectivity.
  - id: failure:k8s/probe-misconfig
    symptom: Pod never receives traffic or is killed while actually healthy.
    cause: Readiness probe too strict (fails during warmup) or liveness probe hitting a slow endpoint causing restart loops.
    fix: Use a lightweight health endpoint, set initialDelaySeconds/periodSeconds generously, and separate liveness from readiness.
  - id: failure:k8s/no-resource-requests
    symptom: Node memory/CPU exhausted; noisy-neighbor evictions; poor scheduling.
    cause: Pods deployed without resource requests so the scheduler overcommits and kubelet cannot reserve capacity.
    fix: Always set CPU/memory requests (and limits) so the scheduler places pods safely and the eviction manager has headroom.
extends:
  - concept:docker/container
uses:
  - concept:docker/dockerfile
  - concept:docker/multistage
  - concept:k8s/pod
  - concept:k8s/deployment
  - concept:k8s/service
  - concept:k8s/probes
  - concept:k8s/resources
part_of: concept:domain/cloud-native
depends_on:
  - package:postgres/patterns
  - package:go/patterns
solves:
  - problem:reproducible-container-orchestration
alternatives:
  - package:hashicorp/nomad
  - package:aws/ecs
  - package:docker/swarm
---
# Docker & Kubernetes Patterns

Containers package an application with its dependencies into an immutable image built by a Dockerfile. The art is keeping that image small, secure, and cache-friendly. Multi-stage builds are the key technique: an early stage installs compilers and builds a binary, then a single `COPY --from=build` copies just the artifact into a slim or distroless final stage, leaving SDKs and source out of production. Layer ordering matters for cache reuse — copy dependency manifests (package.json, go.mod) and run the install before copying source, so code changes don't bust the dependency layer. Never run as root: add a `USER` directive and drop capabilities to shrink the blast radius of a container breakout.

Kubernetes turns individual containers into resilient, self-healing workloads. The Pod is the atomic unit — usually one main container plus optional sidecars sharing network and storage. A Deployment declares how many replicas you want and the pod template; it manages rolling updates and one-command rollbacks via ReplicaSets, so `kubectl rollout undo` is your safety net. Services give pods a stable virtual IP and DNS name selected by labels, so clients don't care which pod instance answers — this decouples consumers from the constant churn of pod lifecycles. Ingress exposes HTTP routing from outside the cluster.

Health is governed by probes. A liveness probe restarts a pod that is dead, while a readiness probe gates traffic until the app can actually serve — set them on different endpoints and give generous `initialDelaySeconds` so warmup doesn't trigger a restart loop. Resource requests and limits are non-negotiable in production: requests let the scheduler reserve capacity and place pods correctly, limits prevent a runaway pod from OOM-killing the node (exit 137) or throttling neighbors. Externalize config with ConfigMaps and secrets with Secrets rather than baking them into images, and use Namespaces to partition a cluster for teams and quotas. The Horizontal Pod Autoscaler scales replicas on CPU/memory/custom metrics (needs metrics-server), and StatefulSets provide stable identities and persistent volumes for databases and queues. Together these patterns yield reproducible images and workloads that survive node failure, traffic spikes, and bad deploys.

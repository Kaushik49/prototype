# ☸️ Practical Implementation of Kubernetes

In a **cloud-native** architecture, applications are deployed within containers rather than being natively installed on a server. This approach decouples the application from the underlying infrastructure.

### Why use Containers?
*   **Environment Consistency:** Containers solve the "it works on my machine" problem.
*   **Dependency Independence:** No need to manually install environments or libraries every time you move from one cloud provider to another.
*   **Portability:** Makes your application truly independent of the host OS.

---

## 🏗️ Container Orchestration

**Orchestration** means managing multiple entities simultaneously. In this context, it is the automated management of container lifecycles.

**The Problem:** In a manual setup, if a container crashes, a developer must manually run the `docker run` command to bring it back up. This is impossible to manage at scale.

Modern applications are composed of various services:
*   **SQL Database:** Postgres
*   **NoSQL Database:** MongoDB
*   **Caching:** Redis
*   **Message Broker:** Kafka

---

## ☸️ What is Kubernetes (K8s)?

Kubernetes is a **Container Orchestration Engine** that allows you to automate the creation, deletion, and updating of containers.

### Key Use Cases:
1.  **Multi-Cloud Mobility:** Easily move workloads from **AWS** to **GCP** (or vice versa).
2.  **Auto-Healing:** You don't need to monitor resources 24/7. If a container goes down, Kubernetes automatically detects it and restarts it.
3.  **Observability & Scaling:** Provides a dashboard to monitor container health and handles **Load Balancing** and **Auto-scaling** based on traffic.

![kubernetes architecture](./images/kubernetes.png)
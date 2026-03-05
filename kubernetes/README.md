# Practical Implementation of kubernetes

In a cloud native fashion you deploy a container in cloud server rather than natively installing the application inside server. In such case, container solves the problem of having to install environment and dependencies each time when you move from one cloud to another. This makes the application depencdency indepent.


## Container Orchestration
Orchestration means to manage different things simultaniously. Container orchestration means , having to manage different containers simultaneously.Incase if the container goes down in any case, the developer needs to run the 'docker run command' if any container goes down in cluster.

An application contains container of different application such as for postgres database, Mongodb for nosql , redis for caching, kafka for high throughput of data etc.

This is where comes Kubernetes comes.
### Kubernetes 
Kubernetes is a container orchestration engine , which lets you create, delete and update containers. 
1. If you want to move from AWS to GCP viceversa
2. If don't worry about patching, want sombody to look at your resources all the time, incase if the container goes down you want sombody to autoheal.
3. If you want want a dashboard of want the container is doing or want to autoscale to some extent, like load balancing.

![kubernetes architecture](./images/kubernetes.png)

the thing that master node starts is a called pod,

![Note]
>pod is not a worker not a container as a single pod can run many containers.Master node is itself a EC2 instance and other worker nodes are EC2 instance as well.

Here what a typical architecture looks like in micro detail. 
![micro_unit](./images/micro_unit.png)

Here was what the subset looks like 
![subet of every entity](./images/subset.png)

## Architecture inside master node
1. Api Server : this where the developer sends HTTP request. The API server exposes http to either start container. It checks if the request sent by developer is authenticated and after authentication , it sends the request to etcd. 
2. Ectd : is a distrubted key-value paid distributed database. If there is multiple master nodes, this database is shared among those multiple master node.
3. Kube scheduler : schedules the job for certain pode in the cluster, finds node for running the container.
4. Controller manger : runs a bunch of controllers which inturns run infinite loop , which checks if something needs to be done.

![Master node architecture](./images/master_node.png)

## Architecture of worker node
1. Kubelets : that checks the request from api server from the master node.
2. Kube-proxy : which handles how you can send http request to the abstraction to container.


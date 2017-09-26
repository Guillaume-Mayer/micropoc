# MicroServices: Concepts & Poc

## Concepts

## Kubernetes

### Dicionario

**Etcd**: Open source Key/Value store distribuido que sirve para **cluster coordination** y **state management**.
> It takes care of storing and replicating data used by Kubernetes across the entire cluster,
> thanks to the Raft consensus algorithm, etcd can recover from hardware failure and network partitions.
> In addition to Kubernetes, Cloud Foundry also uses etcd as their distributed key-value store.

<https://thenewstack.io/about-etcd-the-distributed-key-value-store-used-for-kubernetes-googles-cluster-container-manager/>

<https://github.com/coreos/etcd/>

**KubeCtl**: Cliente (linea de comando)

**KubeLet**: Agente corriendo en cada nodo

**Kubernetes Master**: Conjunto de tres processos corriendo en el node master (uno por cluster)

- kube-apiserver
- kube-controller-manager
- kube-scheduler

**Kubernetes Control Plane**: The various parts of the Kubernetes Control Plane, such as the Kubernetes Master and kubelet processes, govern how Kubernetes communicates with your cluster. The Control Plane maintains a record of all of the Kubernetes Objects in the system, and runs continuous control loops to manage those objects’ state. At any given time, the Control Plane’s control loops will respond to changes in the cluster and work to make the actual state of all the objects in the system match the desired state that you provided.
For example, when you use the Kubernetes API to create a Deployment object, you provide a new desired state for the system. The Kubernetes Control Plane records that object creation, and carries out your instructions by starting the required applications and scheduling them to cluster nodes–thus making the cluster’s actual state match the desired state.

**Kubernetes Nodes**: The nodes in a cluster are the machines (VMs, physical servers, etc) that run your applications and cloud workflows. The Kubernetes master controls each node; you’ll rarely interact with nodes directly.

**Kubernetes Object**:

- Basic objects
  - [Pod] (#pod)
  - Service
  - Volume
  - Namespace
- Controllers (build upon basic objects)
  - ReplicaSet
  - Deployment
  - StatefulSet
  - DaemonSet
  - Job

**Minion**: Slave

<a name="pod"></a>**Pod**: Unidad de construcción en Kubernetes, la mas pequeña y simple unidad que se puede crear y desplegar. En general hay un container por pod pero también pueden ser varios contenedores trabajando juntos.</a>

### Docker vs Kubernetes

Docker | Kubernetes
------ | ----------
docker-cli | kubectl <https://kubernetes.io/docs/user-guide/docker-cli-to-kubectl/>
docker-compose | kompose
docker-machine | ??
docker swarm | ??

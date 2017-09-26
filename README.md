# MicroServices: Concepts & Poc

## Concepts

## Kubernetes

### Dicionario

**Etcd**: Open source Key/Value store distribuido que sirve para **cluster coordination** y **state management**.
> It takes care of storing and replicating data used by Kubernetes across the entire cluster,
> thanks to the Raft consensus algorithm, etcd can recover from hardware failure and network partitions.
> In addition to Kubernetes, Cloud Foundry also uses etcd as their distributed key-value store.
<https://thenewstack.io/about-etcd-the-distributed-key-value-store-used-for-kubernetes-googles-cluster-container-manager/>

**KubeCtl**: Cliente (linea de comando)

**KubeLet**: Agente

**Minion**: Slave

**Pod**: MicroService

### Docker vs Kubernetes

Docker | Kubernetes
------ | ----------
docker-cli | kubectl
docker-compose | kompose
docker-machine | ??
docker swarm | ??

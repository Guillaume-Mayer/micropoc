# Kubernetes

## Glosario

**KubeLet**: Agente corriendo en cada nodo, un nodo es una maquina (virtual o fisica). Kubelet es responsable para levantar los contenedores en el nodo. Se le puede pasar las definiciones de los pods que tiene que correr de varias formas (Archivo, Directory, Http) pero lo mas común es que las traiga del API Server.

**Pod**: Es la mas pequeña y simple unidad que se puede crear y desplegar en Kubernetes. Un pod es un conjunto de contenedores descrito en un pod manifest, tiene una sola IP. Se describe en un manifest yaml o json.

**Etcd**: Es un Key/Value store distribuido, con consistencia fuerte (CP) que sirve para **cluster coordination** y **state management**, es la fuente de verdad del cluster.

> It takes care of storing and replicating data used by Kubernetes across the entire cluster,
> thanks to the Raft consensus algorithm, etcd can recover from hardware failure and network partitions.
> In addition to Kubernetes, Cloud Foundry also uses etcd as their distributed key-value store.

<https://thenewstack.io/about-etcd-the-distributed-key-value-store-used-for-kubernetes-googles-cluster-container-manager/>

<https://github.com/coreos/etcd/>

**API Server**: Es una Api Rest, que se encarga de guardar el estado en Etcd, todos componentes guardan su estado en Etcd a traves del API server.

**KubeCtl**: Es un cliente (linea de comando) que facilita la comunicación con el API Server.

**Kubernetes Master**: Conjunto de tres processos corriendo en el node master (uno por cluster)

- API Server
- Scheduler
- Controller Manager

**Scheduler**: Es el programa que se encarga que afectar un node a un pod que no esta afectado a ninguno, elige de forma inteligente el nodo que afecta al pod, seguún criterias tecnicos (mejor nodo para ese pod) y según una politica que se puede definir.

**Controller Manager**: Se encarga de que se cumplen los requisitos de los objetos tal como el DaemonSet (un pod tiene que correr en cada nodo) o el ReplicaSet (ese pod tiene que ser replicado X veces).

![Scene 1][scene1]

**Kubernetes Object**:

Todo en Kubernetes es decalarativo, todos los objetos se describen en un manifest, en Yaml o Json, el estado de esos objetos se gaurda en Etcd.

- Basic objects
  - Pod
  - Service
  - Volume
  - Namespace
- Controllers (build upon basic objects)
  - ReplicaSet
  - Deployment
  - StatefulSet
  - DaemonSet
  - Job

**Service**: Un servicio es lo que permite acceder a un conjunto de pods de forma unificada, descubriendo los servicios y puertos publicados por los contenedores en los nodos. Mapea los puertos de los multiples servicios a puertos accesibles externamente. Tiene capabilidades como *Load Balancing*, *rolling update*, *label selections*. **Es la noción de Kubernetes que se acerca lo mas de lo que a veces se llama un micro-servicio.**. Se describe en un manifest yaml o json.

Modo **proxy userspce**:

![Service][service-userspace]

Modo **proxy iptables**:

![Service][service-iptables]

**ReplicaSet**: Permite definir cuantos *pods* tendran que ejecutarse. Se describe en un manifest yaml o json.

**Deployment**: Permite describir cambios de *pods* o de *replicaset*, puede controlar otros *replicaset*, *pods* o incluso otros *deployment* objets.
> A Deployment controller provides declarative updates for Pods and ReplicaSets.
> You describe a desired state in a Deployment object, and the Deployment controller changes the actual state to the desired state at a controlled rate. You can define Deployments to create new ReplicaSets, or to remove existing Deployments and adopt all their resources with new Deployments.
Permite rollout de replicaset, rollback de otro deployment, escaleamiento, rollover (multiple actualización al vuelo). Se describe en un manifest yaml o json.

**Job**: Un job crea uno o varios *pods* y se asegura que un numero dado de ellos terminen con exito. Por ejemplo, si un job consista en ejecutar un pod y que ese pod falla por alguna razon, el job creara un nuevo *pod*. Un job puede ejecutar varios *pods* en paralelo. Se describe en un manifest yaml o json.

**DaemonSet**: Un daemonset se asegura que todos los nodos (o parte de ellos) ejecutan un *pod*. Cuando se van agregando nodos al cluster, el pod esta afectado a ellos. Un uso tipico es asegurarse que cada nodo ejecute un colector de logs (Logstash/Fluentd). Se describe en un manifest yaml o json.

![Scene 2][scene2]

**Rolling Update**: Es el hecho de remplazar progresivamente los pods de un servicio por otros pods, con criterios baseados en labels.

**Label Selections**: En los manifiestos de los pods se pueden definir *labels*, que luego permiten de selecionar los *pods* que constituyen un *service*. Sirve por ejemplo para desplegar progresivamente nuevas versiones.

**Kubernetes Control Plane**: Es como un panel de control del cluster (?).
> The various parts of the Kubernetes Control Plane, such as the Kubernetes Master and kubelet processes, govern how Kubernetes communicates with your cluster. > The Control Plane maintains a record of all of the Kubernetes Objects in the system, and runs continuous control loops to manage those objects’ state. At any given time, the Control Plane’s control loops will respond to changes in the cluster and work to make the actual state of all the objects in the system match the desired state that you provided.
> For example, when you use the Kubernetes API to create a Deployment object, you provide a new desired state for the system. The Kubernetes Control Plane records that object creation, and carries out your instructions by starting the required applications and scheduling them to cluster nodes–thus making the cluster’s actual state match the desired state.

**Kubernetes Nodes**: Los nodos del cluster son las maquinas (VMs, physical servers, etc) que ejecutan las aplicaciones y los "cloud workflows" (?). Controla cada nodo; pocas se interactua directamente con los nodos.

**Master**: Server donde corre *API Server*, hay uno por cluster, multi-master esta en la roadmap.

**Minion**: Cada nodo (con un *Kubelet*) que recibe ordenes de un *master*.

## Docker vs Kubernetes

Kubernetes ocupa Docker como runtime de contenedor (via *Kubelet*) y como formato para las imagenes.

Docker | Kubernetes
------ | ----------
docker-cli | kubectl
docker-compose | kubectl
docker-machine | N/A
swarm | cluster
stack | pod
service | service
Dockerfile | Dockerfile
docker-compose.yml | pod manifest

<https://kubernetes.io/docs/user-guide/docker-cli-to-kubectl/>


[scene1]: img/scenes-from-kubernetes-page1.svg "Scenes from Kubernetes"
[scene2]: img/scenes-from-kubernetes-page2.svg "Scenes from Kubernetes"
[service-userspace]: img/services-userspace-overview.svg "UserSpace Mode"
[service-iptables]: img/services-iptables-overview.svg "IPTables Mode"
# Service Discovery

Es la tecnologia que permite que se puedan usar componentes en nodos agregados dinamicamente al sistema.

![Service Discovery][disco]

Es una de las funcionalidades principal de Kubernetes implentado en su nucleo gracias a las tecnologias Etcd y Kubelet así que su noción de Service.

## Noción de servicio en Kubertenes

Permite abstraer la complejidad de que los componentes un servicio se esten ejecutando en varios nodos, proveendo una interface común permitiendo acceder a los nodos como si fuera uno.

## Node discovery

En Kubernetes, cada nodo se registra via *Kubelet* en el API Server/Etcd como miembro del cluster.

[disco]: ../img/service-discovery.png "Service Discovery Pronblem"
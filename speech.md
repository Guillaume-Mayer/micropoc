# Architectura en microservicios con contenedores

El modelo "Mesh and Service Architecture" (MASA) recomienda ocupar el padrón "Micro Services" y contenerización.

## Microservices

Es un padrón de diseño de sistemas informaticos que recomienda separar las funcionalidades en unidades independientes llamadas microservicios.

## Contenerización

Es un conjunto de tecnologias (LXD, namespace, sysgroup, Docker) que permiten paquetear aplicaciones en un entorno aislado parecido a una máquina virtual pero mucho mas liviano y **orientado a aplicacíon**. Pertenece también a la tendencia **DevOps** porque facilita mucho el despliegue y la automatización.

## Kubernetes

Kubernetes es un ecosistema **OpenSource** procedente de la experiencia de Google en los contenedores, su proposito principal es de abstraer la complejidad de la infraestructura (cluster de máquinas virtuales, fisicas o en el Cloud), uno le pide de ejecutar un servicio sin importarse donde corre ese servicio.

Es capaz de manejar *contenedores* Docker como *Cloud-Native* components.

### Kubernetes cubre las necesidades siguientes

* Service Discovery
* Load Balancing
* Auto Scaling
* Self Healing
* Resilience (Fault Tolerance)
* API Gateway
* High Availability
* Centralized Configuration Management
* Cloud Native
* Monitoring

### Necesidades que no cubre Kubernetes

* HATEOAS: Nivel aplicativo
* Centralized Logging: EFK/ELK
* Centralized Metrics: Grafana
* Distributed Tracing: OpenTracing, Zipkin
* CI/CD: Jenkins + plugins

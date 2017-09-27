# Architectura en microservicios con contenedores

El modelo "Mesh and Service Architecture" (MASA) recomienda ocupar el padrón "Micro Services" y contenerización.

## Microservices

Es un padrón de diseño de sistemas informaticos que recomienda separar las funcionalidades en unidades independientes llamadas microservicios.

## Contenerización

Es un conjunto de tecnologias (LXD, namespace, sysgroup, Docker) que permiten paquetear aplicaciones en un entorno aislado parecido a una máquina virtual pero mucho mas liviano y **orientado a aplicacíon**.

## Kubernetes

Kubernetes es un ecosistema **OpenSource** procedente de la experiencia de Google en los contenedores, su proposito principal es de abstraer la complejidad de la infraestructura (cluster de máquinas virtuales, fisicas o en el Cloud), uno le pide de ejecutar un servicio sin importarse donde corre ese servicio.

Kubernetes cubre las necesidades siguientes:

* Service Discovery
* Load Balancing
* API Gateway
* Resilience (Fault Tolerance)
* Self Healing
* High Availability
* Centralized Configuration Management
* Cloud Native
* Monitoring
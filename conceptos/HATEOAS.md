# Hypertext As The Engine Of Application State (HATEOAS)

Es una prolongación de los conceptos de REST.
Es el nivel maximo de RESTFullness que puede cumplir un API.

## Implicaciones

- Aplicar los estandares HTTP en la API (REST)
- Aplicar las "best practices" de REST
- Las respuestas devuelven los endpoints de las aciones posibles, permitiendo el *auto-descubrimiento* de la API.

## Dominio: Aplicación

Es la responsabilidad de cada aplicación de implemenar los estandares HATEOAS.

## Soluciones

Respetar HATEOAS en el desarrollo de los servicios REST (responsabilidad del desarrollo).
**Es dificil recomendar una solución tecnica en un contexto donde cada microservice puede y quiere ser desarrollado en lenguajes diferentes.**

## Frameworks

Varios frameworks tienen soporte HATEOAS:

- Spring HATEOAS part of the Spring Framework (Java) <http://projects.spring.io/spring-hateoas/>
- Yii Framework (PhP) <http://www.yiiframework.com/doc-2.0/guide-rest-quick-start.html>
- Eve Framework (Python) <http://python-eve.org/features.html>
- Baucis for NodeJS <https://github.com/wprl/baucis>

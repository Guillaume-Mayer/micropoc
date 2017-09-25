# Search Service

Microservice de busqueda de productos

* Rest Api: NodeJS + Express
* Store : Elastic Search <https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html>

## Buscar productos

* <http://localhost:3000/search/martillo>
* <http://localhost:3000/search/clavos>
* <http://localhost:3000/search/caja>
* <http://localhost:3000/search/destor*>
* <http://localhost:3000/search/*ill*>

## Agregar un producto

### Ejemplo

POST http://localhost:3000/product

{
    "id": "CC200",
    "name": "Caja de 200 clavos",
    "price": 1800,
    "tags": ["bricolaje", "herramienta"]
}

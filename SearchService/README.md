# Search Service

With Elastic Search <https://www.elastic.co/guide/en/elasticsearch/reference/current/docker.html>

## Buscar productos

<http://localhost:3000/search/caja>
<http://localhost:3000/search/torni>

## Agregar un producto

POST http://localhost:3000/product
{
    "id": "CC200",
    "name": "Caja de 200 clavos",
    "price": 1800,
    "tags": ["bricolaje", "herramienta"]
}
# Order Service

## Responisbilidades del servicio

* Verificar que hay stock para el producto pedido
* Guardar los orden en base de datos
* Permitir al cliente de ver el estatus de su orden

## Stack

* Api: Rust + Iron
* Store: PostgreSql

## Pasar un orden

POST <http://localhost:3002/order>

{
    "product": "M1",
    "qty": 1,
    "client": {
        "id": 12345,
        "name": "Guillaume Mayer",
        "email": "gmayer@arkho.tech",
        "address": {
            "comuna": "Las Condes",
            "direccion": "Hamlet 4340",
            "depto": "712"
        }
    }
}

## Obtener el estatus de un orden

GET <http://localhost:3002/order/1>

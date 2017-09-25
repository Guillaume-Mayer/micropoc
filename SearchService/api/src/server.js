// Modules
const express = require('express')
const morgan = require('morgan')
const bodyParser = require('body-parser')
const elasticsearch = require('elasticsearch')

// Express app
const app = express()

// Log
app.use(morgan('tiny'))

// Parser Json
app.use(bodyParser.json())

// List of all products
app.get('/products', function(req, res) {
	res.json([{
        id: "M1",
        name: "Martillo",
        price: 14500,
        tags: ["herramienta", "bricolaje"]
    }, {
        id: "CC100",
        name: "Caja de 100 clavos",
        price: 900,
        tags: ["accesorio", "bricolaje"]
      }, {
        id: "CC200",
        name: "Caja de 200 clavos",
        price: 1800,
        tags: ["accesorio", "bricolaje"]
      }, {
        id: "D1",
        name: "Destornillador",
        price: 11500,
        tags: ["herramienta", "bricolaje"]
      }, {
        id: "CT300",
        name: "Caja de 300 tornillos",
        price: 3000,
        tags: ["accesorio", "bricolaje"]
    }])
})

// Elastic search
app.get('/search/:name', function(req, res) {
  searchByName(req.params.name, function(err, hits) {
    if (err) {
      res.status(500).end()
    } else {
      res.json(hits.hits)
    }
  })
})

// Add a product
app.post('/product', function(req, res) {
  let product = req.body
  if (product && product.id && product.name && product.price) {
    index(product)
    res.status(201).json(product)
  } else {
    res.status(403).json({
        err: 101,
        msg: "Id, Name y Price are required"
    })
  }
})

// Start the server
const port = 3000
app.listen(port)
console.log('Server running at port ' + port)

// Elastic Search Client
const client = new elasticsearch.Client({
    host: 'http://elastic:changeme@store:9200',
    log: 'info'
})

// Index a product
function index(product) {
  client.index({
    index: 'products',
    type: 'product',
    body: {
        id: product.id,
        name: product.name,
        precio: product.precio,
        tags: product.tags,
        published: true,
    }}, function (error, response) {
    if (error) {
      console.error(error);
    } else {
      console.log(response)
    }
  })
}

// Search a product by name
function searchByName(name, callback) {
  client.search({
      index: 'products',
      type: 'product',
      q: 'name:' + name
    }, callback)
}
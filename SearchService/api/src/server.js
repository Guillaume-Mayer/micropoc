// Modules
const express = require('express')
const morgan = require('morgan')

// Express app
const app = express()

// Log
app.use(morgan('tiny'))

// List of all products
app.get('/', function(req, res) {
	res.json([{
        id: 1,
        name: "Martillo",
        precio: 8500
    }, {
        id: 2,
        name: "Caja de 100 clavos",
        precio: 900
    }])
})

// Add a product
app.post('/', function(req, res) {
    res.status(201).end()
})

// Start the server
const port = 3000
app.listen(port)
console.log('Server running at port ' + port)
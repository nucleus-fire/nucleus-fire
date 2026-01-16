const fastify = require('fastify')({ logger: false })

fastify.get('/', async (request, reply) => {
  return 'Hello World'
})

fastify.listen({ port: 3004, host: '0.0.0.0' }, (err, address) => {
  if (err) {
    fastify.log.error(err)
    process.exit(1)
  }
  console.log(`Fastify listening on ${address}`)
})

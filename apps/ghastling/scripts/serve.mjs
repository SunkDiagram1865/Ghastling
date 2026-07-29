import { createServer } from 'node:http'
import { readFileSync, existsSync, statSync } from 'node:fs'
import { join, extname, normalize } from 'node:path'

const PORT = Number(process.env.PORT) || 3000
const ROOT = normalize(process.argv[2] || '.')

const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.mjs': 'application/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.webp': 'image/webp',
  '.ico': 'image/x-icon',
  '.ogg': 'audio/ogg',
  '.gltf': 'model/gltf+json',
  '.webmanifest': 'application/manifest+json',
  '.txt': 'text/plain',
  '.xml': 'application/xml',
}

const server = createServer((req, res) => {
  let path = decodeURIComponent(req.url.split('?')[0])
  if (path === '/') path = '/index.html'
  const filePath = join(ROOT, path)
  
  if (!existsSync(filePath) || !statSync(filePath).isFile()) {
    res.writeHead(404, { 'Content-Type': 'text/plain' })
    res.end('Not Found: ' + path)
    return
  }
  
  const ext = extname(filePath).toLowerCase()
  const contentType = MIME_TYPES[ext] || 'application/octet-stream'
  
  const content = readFileSync(filePath)
  res.writeHead(200, {
    'Content-Type': contentType,
    'Cache-Control': 'no-cache, no-store, must-revalidate',
    'Access-Control-Allow-Origin': '*',
  })
  res.end(content)
})

server.listen(PORT, () => {
  console.log(`Static server running at http://localhost:${PORT}`)
  console.log(`Serving files from: ${ROOT}`)
})
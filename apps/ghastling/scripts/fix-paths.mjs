import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs'
import { join, extname } from 'node:path'

const publicDir = process.argv[2] || join(import.meta.dirname, '..', '.output', 'public')

function fixHtmlPaths(filePath) {
  let content = readFileSync(filePath, 'utf-8')
  // 把 href="/..." 和 src="/..." 改成相对路径 "./..."
  content = content.replace(/(href|src)="\/(?!\/)/g, '$1="./')
  // 修改内联脚本中的 baseURL:"/" 为 baseURL:"./"
  content = content.replace(/baseURL:"\/"/g, 'baseURL:"./"')
  writeFileSync(filePath, content, 'utf-8')
}

function walkDir(dir) {
  for (const entry of readdirSync(dir)) {
    const fullPath = join(dir, entry)
    const stat = statSync(fullPath)
    if (stat.isDirectory()) {
      walkDir(fullPath)
    } else if (extname(fullPath) === '.html') {
      fixHtmlPaths(fullPath)
      console.log(`Fixed: ${fullPath}`)
    }
  }
}

walkDir(publicDir)
console.log('Done! All HTML files patched to use relative paths.')

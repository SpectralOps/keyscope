const L = require('lodash')
const fs = require('fs')
const file = fs.readFileSync('src/defs.yaml', 'utf8')
const YAML = require('yaml')
const defs = YAML.parse(file)


const rows = L.map(defs.providers, (def, provider)=>{
	return [provider,  def.validation.request.desc, 'validation', def.validation.request.params.map(p=> `\`${p.name}\` - ${p.desc}`).join("<br/>")]
})

const header = `| provider | actions | params |\n|---|---|---|\n`
const table = header + L.map(rows, ([p, desc, a, prm])=> `|**${p}**<br/>${desc}|${a}|${prm}|`).join("\n")
const readme = fs.readFileSync('README.md', 'utf8').toString()
const out = readme.replace(/<!-- providers -->([\S\s]*?)<!-- \/providers -->/g, `<!-- providers -->\n${table}\n<!-- \/providers -->\n`)
fs.writeFileSync("README.md", out)
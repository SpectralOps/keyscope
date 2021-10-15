const L = require('lodash')
const fs = require('fs')
const file = fs.readFileSync('src/defs.yaml', 'utf8')
const YAML = require('yaml')
const defs = YAML.parse(file)


const rows = L.map(defs.providers, (def, provider)=>{
	return [
		provider,  
		def.validation.request.desc, 
		'validation', 
		def.validation.request.params.map(p=> `\`${p.name}\` - ${p.desc}`).join("<br/>"),  
		"```\n"+ `keyscope validate ${provider} -p ${def.validation.request.params.map(p=> p.name.toUpperCase()).join(" ") }` +"\n```"
	]
})

const table = "<table>\n" + L.map(rows, ([p, desc, a, prm, code])=> `<tr><td>\n\n**${p}**<br/>${desc}\n\n</td>\n<td>\n\n${a}\n\n</td>\n<td>\n\n${prm}\n\n</td>\n</tr>\n<tr>\n<td colspan="3">\n\n${code}\n</td></tr>`).join("\n") + "\n</table>"
const readme = fs.readFileSync('README.md', 'utf8').toString()
const out = readme.replace(/<!-- providers -->([\S\s]*?)<!-- \/providers -->/g, `<!-- providers -->\n${table}\n<!-- \/providers -->\n`)
fs.writeFileSync("README.md", out)
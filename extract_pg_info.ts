import { DOMParser, Element } from "jsr:@b-fuze/deno-dom@^0.1.56"

const url = Deno.args[0]
if (url === undefined) {
	console.error("Usage: deno run --allow-net extract_pg_table.ts <url>")
	Deno.exit(1)
}

const response = await fetch(url)
if (!response.ok)
	throw new Error(`HTTP ${response.status}: ${response.statusText}`)

const html = await response.text()
const doc = new DOMParser().parseFromString(html, "text/html")
if (doc === null)
	throw new Error("Failed to parse HTML document.")

// Find the first table.table that contains catalog_table_entry cells
let targetTable: Element | null = null
for (const table of doc.querySelectorAll("table.table")) {
	if (table.querySelector("td.catalog_table_entry") !== null) {
		targetTable = table
		break
	}
}
if (!targetTable)
	throw new Error("No catalog table found on the page.")

for (const row of targetTable.querySelectorAll("tbody tr")) {
	const td = row.querySelector("td.catalog_table_entry")
	if (td === null) throw new Error("no catalog_table_entry")

	const colDef = td.querySelector("p.column_definition")
	if (colDef === null) throw new Error("no column_definition")

	const fieldEl = colDef.querySelector("code.structfield")
	const typeEl = colDef.querySelector("code.type")
	if (fieldEl === null || typeEl === null) throw new Error("no structfield or type")

	const colName = fieldEl.textContent.trim()
	const colType = typeEl.textContent.trim()

	// Collect all <p> elements that are NOT the column_definition paragraph
	const description = Array.from(td.querySelectorAll("p"))
		.filter((p) => !p.classList.contains("column_definition"))
		.map((p) => (p.textContent ?? "").trim().replace(/\s+/g, " "))
		.join(" ")

	console.log(`${colName} ${colType} -- ${description}`)
}

import mariadb from "npm:mariadb@^3.4.5"

const password = prompt("db password:")
const host = prompt(
	"db network (probably database for podman or localhost for docker):"
)
const oldurl = prompt("old url:")
const newurl = prompt("new url:")

const pool = mariadb.createPool({
	host,
	password,
	user: "akyuu",
	database: "akyuu_records",
	aquireTimeout: 5000,
})

let conn = await pool.getConnection();
await conn.query("START TRANSACTION;")

const moves = await conn.query("SELECT * FROM `moves`;")

moves.forEach(async move => {
	move.data.variations.forEach( v => {
		if (v.attachment) v.attachment = v.attachment.replace(oldurl, newurl)
	})

	const res = await conn.query(
		"UPDATE `moves` "
		+ "SET `data`=JSON_COMPACT('" + JSON.stringify(move.data) + "') "
		+ "WHERE `id`=" + move.id + ";"
	)

	if (res.affectedRows != 1) {
		console.log("ERROR WITH MOVE ID: " + move.id)
		console.log("EXITING")
		await conn.query("ROLLBACK;")
		if (conn) conn.release()
		process.exit()
	}
})

await conn.query("COMMIT;")
if (conn) conn.release()
console.log("updated " + moves.length + " rows")
process.exit()

from sqlite3 import connect
#>-------------------------------<#
# to comunicate with the database #
#>-------------------------------<#
def write_db(ip, location):
	con = connect('communicate.db')
	cur = con.cursor()
	try:
		cur.execute('''CREATE TABLE ips
				(ip text, location text)''')
	except:
		pass
	cur.execute(f"REPLACE INTO ips VALUES ('{ip}','{location}')")
	con.commit()
	con.close()
def read_db(table) -> list:
	con = connect('communicate.db')
	content = []
	cur = con.cursor()
	try:
		for c in cur.execute(f"SELECT * FROM {table}"):
			content.append(c)
	except Exception as e:
		print("error", e)
	con.close()
	return content

from scapy.all import *
from time import sleep
from pathlib import Path
from ip2geotools.databases.noncommercial import DbIpCity
from json import load

from sqlite3 import connect

#>-------------------------------
# to comunicate with the database
#>-------------------------------
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
#>-------------------------------
# IP Stuff
#>-------------------------------
def pc(packet):
	if packet.proto == 17:
		udp = packet.payload


home = str(Path.home())

db_cont = read_db("ips")
test_z = [i[0] for i in db_cont]
print(f"test_z = {test_z}")
jdata = {}

with open("scripts/assets/ISO3166-1.alpha2.json") as f:
	jdata = load(f)

def get_location(ip:str) -> str:
	try:
		country_code = DbIpCity.get(ip, api_key='free').country
		#print(country_code)
		country = f"{jdata[country_code]}*[{country_code}]"
		#print(country)
	except Exception as e:
		country = "localization_faild_country"
	try:
		region = DbIpCity.get(ip, api_key='free').region
	except:
		region = "localization_faild_region"
	try:
		city = DbIpCity.get(ip, api_key='free').city
	except:
		city = "localization_faild_city"


	return f"{country}+{region}+{city}"


def main():
	i=0
	while i <= 2:
		x = sniff(filter="udp and port 6672", prn=pc, store=1, count=1)			# GTA V Online UDP default Port is 6672
		y = x[0][IP].src
		z = x[0][IP].dst

		if z == "192.168.1.103":												#replace with your local IP
			pass
		else:
			print("-----------------------------------------------------------")
			try:
				#print(f"Destination: IP Address (z): [{z}] ")
				#print(f"y: IP Address (y): [{y}] ")

				#[print("x: ", x[0]) for x in test_z]
				#[print("z: ", z)]

				#print(z in x[0] for x in test_z)
				if z in test_z:
					pass
				else:
					#print(f"{z}")
					write_db(z, get_location(z).replace(" ", "_"))
					test_z.append(z)
			except Exception as e:
				print(e)
		i += 1

main()
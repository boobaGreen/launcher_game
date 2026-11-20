import json
import sqlite3

# 1. Update data/master_cloud_catalog.json
catalog_path = 'data/master_cloud_catalog.json'
with open(catalog_path, 'r', encoding='utf-8-sig') as f:
    cloud = json.load(f)

changed_cloud = 0
for g in cloud:
    if g.get('platform') == 'LaserDisc Arcade':
        g['platform'] = 'Laserdisc Arcade'
        changed_cloud += 1

if changed_cloud > 0:
    with open(catalog_path, 'w', encoding='utf-8') as f:
        json.dump(cloud, f, indent=2, ensure_ascii=False)
    print(f'Normalized {changed_cloud} entries in master_cloud_catalog.json')

# 2. Update retro_hub.db
conn = sqlite3.connect('retro_hub.db')
c = conn.cursor()
c.execute("UPDATE games SET platform = 'Laserdisc Arcade' WHERE platform = 'LaserDisc Arcade'")
conn.commit()
print('Normalized in retro_hub.db, rows affected:', c.rowcount)
conn.close()

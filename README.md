
## Initialize MongoDB Data
### Through Script
```
podman run -it --userns keep-id:uid=999,gid=999 -p 27017:27017 -e MONGO_INITDB_DATABASE=account -v $PWD/data/initdb.js:/data/db/initdb.js mongo

```

### Manual Exec
```
podman exec -it <ps_id> /bin/bash

mongo

use <db_name>;

db.createCollection('info', { capped: false });

db.info.save(
...     [
...         {
...             holder_name: "Haldiram",
...             status: "ACTIVE"
...         },
...         {
...             holder_name: "Nirulas",
...             status: "ACTIVE"
...         }
...     ]
... );
```


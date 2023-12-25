db = db.getSiblingDB('account');
db.createCollection('info', { capped: false });

db.info.save(
    [
        {
            holder_name: "Haldiram",
            status: "ACTIVE"
        },
        {
            holder_name: "Nirulas",
            status: "ACTIVE"
        }
    ]
);

db.info.save(
    {
        holder_name: "Baskin & Robbins",
        status: "ACTIVE"
    }
);
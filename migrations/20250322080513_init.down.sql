DELETE FROM
    users
WHERE
    id IN (1, 2);

DROP TABLE IF EXISTS users;

DROP extension IF EXISTS "uuid-ossp";

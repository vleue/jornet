CREATE TABLE admins(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   name TEXT NOT NULL UNIQUE
);

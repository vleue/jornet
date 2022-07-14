CREATE TABLE admins_github(
   id INT NOT NULL,
   PRIMARY KEY (id),
   login TEXT NOT NULL,
   admin_id UUID NOT NULL
);

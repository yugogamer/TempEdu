CREATE EXTENSION pgcrypto;
CREATE EXTENSION "uuid-ossp";


CREATE TABLE IF NOT EXISTS accounts(
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    abreviate_name VARCHAR(5) UNIQUE NOT NULL,
    mail TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS groupes(
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    protected BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS roles(
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS permissions (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS matieres(
    id SERIAL PRIMARY KEY,
    name TEXT,
    description TEXT
);

CREATE TABLE IF NOT EXISTS weeks(
    id SERIAL PRIMARY KEY,
    week INTEGER NOT NULL,
    year INTEGER NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    visible BOOLEAN NOT NULL DEFAULT false,
    CHECK (week > 0),
    CHECK (week < 53),
    CHECK (year > 0),
    CHECK (start_time < end_time)
);

CREATE TABLE IF NOT EXISTS creneaux(
    id SERIAL PRIMARY KEY,
    id_week INTEGER NOT NULL REFERENCES weeks,
    id_matiere INTEGER REFERENCES matieres,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    name VARCHAR(50),
    description TEXT,
    CHECK (start_time < end_time)
);

CREATE TABLE IF NOT EXISTS accountsToGroupes(
    id_account INTEGER REFERENCES accounts,
    id_groupe INTEGER REFERENCES groupes,
    PRIMARY KEY (id_account, id_groupe)
);

CREATE TABLE IF NOT EXISTS accountsToMatieres(
    id_account INTEGER REFERENCES accounts,
    id_matiere INTEGER REFERENCES matieres,
    PRIMARY KEY (id_account, id_matiere)
);

CREATE TABLE IF NOT EXISTS accountsToCreneaux(
    id_account INTEGER REFERENCES accounts,
    id_creneau INTEGER REFERENCES creneaux,
    PRIMARY KEY (id_account, id_creneau)
);

CREATE TABLE IF NOT EXISTS groupesToCreneaux(
    id_creneau INTEGER REFERENCES creneaux,
    id_groupe INTEGER REFERENCES groupes,
    PRIMARY KEY (id_groupe, id_creneau)
);

CREATE TABLE IF NOT EXISTS roleToUsers(
    id_role INTEGER REFERENCES roles,
    id_user INTEGER REFERENCES accounts,
    PRIMARY KEY (id_role, id_user)
);

CREATE TABLE IF NOT EXISTS roleToPermissions(
    id_role INTEGER REFERENCES roles,
    id_permission INTEGER REFERENCES permissions,
    PRIMARY KEY (id_role, id_permission)
);

INSERT INTO roles (id, name, description) VALUES (1, 'admin', 'Administrateur');
INSERT INTO roles (id ,name, description) VALUES (2, 'editor', 'Editor');
INSERT INTO roles (id, name, description) VALUES (3, 'user', 'User');

INSERT INTO permissions (name, description) VALUES ('edit_edt', 'Create and edit a crenaux');
INSERT INTO permissions (name, description) VALUES ('create_account', 'Create an account');
INSERT INTO permissions (name, description) VALUES ('see_all_account', 'view all account');
INSERT INTO permissions (name, description) VALUES ('see_protected_groupe', 'See all groupes');
INSERT INTO permissions (name, description) VALUES ('see_invisible_week', 'See all week');
INSERT INTO permissions (name, description) VALUES ('edit_role', 'Edit a role');

INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 1);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 2);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 3);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 4);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 5);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (1, 6);

INSERT INTO roleToPermissions (id_role, id_permission) VALUES (2, 1);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (2, 3);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (2, 4);
INSERT INTO roleToPermissions (id_role, id_permission) VALUES (2, 5);
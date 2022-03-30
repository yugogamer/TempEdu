CREATE EXTENSION pgcrypto;

CREATE TABLE IF NOT EXISTS accounts(
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    mdp TEXT NOT NULL
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    abreviate_name VARCHAR(5) UNIQUE NOT NULL,
    mail TEXT UNIQUE NOT NULL,
);

CREATE TABLE IF NOT EXISTS groupes(
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
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

CREATE TABLE IF NOT EXISTS session(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_user INTEGER NOT NULL REFERENCES accounts,
    expiration_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW() + INTERVAL '31 day',
)

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
    id_account INTEGER REFERENCES accounts,
    PRIMARY KEY (id_account, id_creneau)
);
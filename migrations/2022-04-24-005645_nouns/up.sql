CREATE TABLE nouns (
    gender varchar(255) NOT NULL,
    declension varchar(255) NOT NULL,
    n_sg varchar(255) NOT NULL PRIMARY KEY,
    g_sg varchar(255) NOT NULL,
    d_sg varchar(255) NOT NULL,
    acc_sg varchar(255) NOT NULL,
    ab_sg varchar(255) NOT NULL,
    voc_sg varchar(255) NOT NULL,
    n_pl varchar(255) NOT NULL,
    g_pl varchar(255) NOT NULL,
    d_pl varchar(255) NOT NULL,
    acc_pl varchar(255) NOT NULL,
    ab_pl varchar(255) NOT NULL,
    voc_pl varchar(255) NOT NULL,
    translation varchar(255) NOT NULL,
    def varchar(511) NOT NULL,
    CONSTRAINT UC_Noun UNIQUE (n_sg)
);
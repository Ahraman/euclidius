CREATE TABLE IF NOT EXISTS pages ( 
    page_id SERIAL PRIMARY KEY NOT NULL,
    page_title VARCHAR(255) NOT NULL UNIQUE,

    page_rev INTEGER NOT NULL
        REFERENCES revisions (rev_id)
);

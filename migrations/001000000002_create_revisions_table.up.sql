CREATE TABLE IF NOT EXISTS revisions ( 
    rev_id SERIAL PRIMARY KEY NOT NULL,
    rev_parent INTEGER DEFAULT NULL
        REFERENCES revisions (rev_id),

    rev_content TEXT NOT NULL DEFAULT '',

    rev_page INTEGER DEFAULT NULL,
    rev_user INTEGER NOT NULL
        REFERENCES users (user_id),

    rev_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS Rune_Pool_Data_Meta (
        id SERIAL PRIMARY KEY,
        startTime BIGINT,
        endTime BIGINT,
        startUnits BIGINT,
        startCount BIGINT,
        endUnits BIGINT,
        endCount BIGINT
);


 CREATE TABLE IF NOT EXISTS Rune_Pool_Data_Intervals (
        id SERIAL PRIMARY KEY,
        startTime BIGINT,
        endTime BIGINT,
        count BIGINT,
        units BIGINT
);
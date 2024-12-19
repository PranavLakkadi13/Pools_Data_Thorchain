-- Table for PoolData
CREATE TABLE IF NOT EXISTS Earning_Data_Pool_Data (
    id SERIAL PRIMARY KEY,
    pool TEXT NOT NULL,
    asset_liquidity_fees NUMERIC(38, 0) NOT NULL,
    rune_liquidity_fees NUMERIC(38, 0) NOT NULL,
    total_liquidity_fees_rune NUMERIC(38, 0) NOT NULL,
    saver_earning NUMERIC(38, 0) NOT NULL,
    rewards NUMERIC(38, 0) NOT NULL,
    earnings NUMERIC(38, 0) NOT NULL
);

-- Table for RunePoolMeta
CREATE TABLE Earning_Data_Rune_Pool_Meta (
    id SERIAL PRIMARY KEY,
    start_time NUMERIC(38, 0) NOT NULL,
    end_time NUMERIC(38, 0) NOT NULL,
    liquidity_fees NUMERIC(38, 0) NOT NULL,
    block_rewards NUMERIC(38, 0) NOT NULL,
    earnings NUMERIC(38, 0) NOT NULL,
    bonding_earnings NUMERIC(38, 0) NOT NULL,
    liquidity_earnings NUMERIC(38, 0) NOT NULL,
    avg_node_count DOUBLE PRECISION NOT NULL,
    rune_price_usd DOUBLE PRECISION NOT NULL
);

-- Table for RunePoolInterval
CREATE TABLE Earning_Data_Rune_Pool_Interval (
    id SERIAL PRIMARY KEY,
    start_time NUMERIC(38, 0) NOT NULL,
    end_time NUMERIC(38, 0) NOT NULL,
    liquidity_fees NUMERIC(38, 0) NOT NULL,
    block_rewards NUMERIC(38, 0) NOT NULL,
    earnings NUMERIC(38, 0) NOT NULL,
    bonding_earnings NUMERIC(38, 0) NOT NULL,
    liquidity_earnings NUMERIC(38, 0) NOT NULL,
    avg_node_count DOUBLE PRECISION NOT NULL,
    rune_price_usd DOUBLE PRECISION NOT NULL
);

-- Junction table for the relationship between RunePoolMeta and PoolData
CREATE TABLE Earning_Data_Meta_Pools (
    meta_id INT REFERENCES earning_data_rune_pool_meta(id),
    pool_id INT REFERENCES earning_data_pool_data(id),
    PRIMARY KEY (meta_id, pool_id)
);

-- Junction table for the relationship between RunePoolInterval and PoolData
CREATE TABLE Earning_Data_Interval_Pools (
    interval_id INT REFERENCES earning_data_rune_pool_interval(id),
    pool_id INT REFERENCES earning_data_pool_data(id),
    PRIMARY KEY (interval_id, pool_id)
);

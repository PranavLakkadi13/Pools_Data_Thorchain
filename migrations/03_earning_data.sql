-- Table for PoolData
CREATE TABLE IF NOT EXISTS earning_data_pool_data (
    id SERIAL PRIMARY KEY,
    pool TEXT NOT NULL,
    asset_liquidity_fees BIGINT NOT NULL,
    rune_liquidity_fees BIGINT NOT NULL,
    total_liquidity_fees_rune BIGINT NOT NULL,
    saver_earning BIGINT NOT NULL,
    rewards BIGINT NOT NULL,
    earnings BIGINT NOT NULL
);

-- Table for RunePoolMeta
CREATE TABLE IF NOT EXISTS earning_data_rune_pool_meta (
    id SERIAL PRIMARY KEY,
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    liquidity_fees BIGINT NOT NULL,
    block_rewards BIGINT NOT NULL,
    earnings BIGINT NOT NULL,
    bonding_earnings BIGINT NOT NULL,
    liquidity_earnings BIGINT NOT NULL,
    avg_node_count FLOAT NOT NULL,
    rune_price_usd FLOAT NOT NULL
);

-- Table for RunePoolInterval
CREATE TABLE IF NOT EXISTS earning_data_rune_pool_interval (
    id SERIAL PRIMARY KEY,
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    liquidity_fees BIGINT NOT NULL,
    block_rewards BIGINT NOT NULL,
    earnings BIGINT NOT NULL,
    bonding_earnings BIGINT NOT NULL,
    liquidity_earnings BIGINT NOT NULL,
    avg_node_count FLOAT NOT NULL,
    rune_price_usd FLOAT NOT NULL
);

-- Junction table for the relationship between RunePoolMeta and PoolData
CREATE TABLE IF NOT EXISTS earning_data_meta_pools (
    meta_id INT REFERENCES earning_data_rune_pool_meta(id),
    pool_id INT REFERENCES earning_data_pool_data(id),
    PRIMARY KEY (meta_id, pool_id)
);

-- Junction table for the relationship between RunePoolInterval and PoolData
CREATE TABLE IF NOT EXISTS earning_data_interval_pools (
    interval_id INT REFERENCES earning_data_rune_pool_interval(id),
    pool_id INT REFERENCES earning_data_pool_data(id),
    PRIMARY KEY (interval_id, pool_id)
);

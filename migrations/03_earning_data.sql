-- Table for RunePoolInterval (must be created first since it's referenced by pool_data)
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


-- Table for PoolData with interval reference
CREATE TABLE IF NOT EXISTS earning_data_pool_data (
    id SERIAL PRIMARY KEY,
    interval_id INTEGER NOT NULL,
    pool TEXT NOT NULL,
    asset_liquidity_fees BIGINT NOT NULL,
    rune_liquidity_fees BIGINT NOT NULL,
    total_liquidity_fees_rune BIGINT NOT NULL,
    saver_earning BIGINT NOT NULL,
    rewards BIGINT NOT NULL,
    earnings BIGINT NOT NULL,
    FOREIGN KEY (interval_id) REFERENCES earning_data_rune_pool_interval(id) ON DELETE CASCADE
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

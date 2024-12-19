CREATE TABLE IF NOT EXISTS Rune_Pool_Depth_Meta (
    id SERIAL PRIMARY KEY,
    startTime BIGINT,
    endTime BIGINT,
    priceShiftLoss NUMERIC(10, 8),
    luviIncrease NUMERIC(10, 8),
    startAssetDepth BIGINT,
    startRuneDepth BIGINT,
    startLPUnits BIGINT,
    startMemberCount BIGINT,
    startSynthUnits BIGINT,
    endAssetDepth BIGINT,
    endRuneDepth BIGINT,
    endLPUnits BIGINT,
    endMemberCount BIGINT,
    endSynthUnits BIGINT
);

CREATE TABLE IF NOT EXISTS Rune_Pool_Depth_Intervals (
    id SERIAL PRIMARY KEY,
    startTime BIGINT,
    endTime BIGINT,
    assetDepth BIGINT,
    runeDepth BIGINT,
    assetPrice NUMERIC(10, 8),
    assetPriceUSD NUMERIC(10, 8),
    liquidityUnits BIGINT,
    membersCount BIGINT,
    synthUnits BIGINT,
    synthSupply BIGINT,
    units BIGINT,
    luvi NUMERIC(10, 8)
);
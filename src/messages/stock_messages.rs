#[derive(Debug)]
pub enum MarketCategory {
    GlobalSelectMarket,
    GlobalMarket,
    CapitalMarket,
    NYSE,
    NyseAmerican,
    NyseArca,
    BatsZExchange,
    InvestorsExchange,
    Unknown,
}


impl MarketCategory {

    fn new(symbol: &char) -> Self {
        match symbol {
            'Q' => Self::GlobalSelectMarket,
            'G' => Self::GlobalMarket,
            'S' => Self::CapitalMarket,
            'N' => Self::NYSE,
            'A' => Self::NyseAmerican,
            'P' => Self::NyseArca,
            'Z' => Self::BatsZExchange,
            'V' => Self::InvestorsExchange,
            _   => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GlobalSelectMarket => "Global Select Market",
            Self::GlobalMarket       => "Global Market",
            Self::CapitalMarket      => "Capital Market",
            Self::NYSE               => "NYSE",
            Self::NyseAmerican       => "NYSE American",
            Self::NyseArca           => "NYSE Arca",
            Self::BatsZExchange      => "Bats Z Exchange",
            Self::InvestorsExchange  => "Investors Exchange",
            Self::Unknown            => "Unknown market",
        }
    }

}

#[derive(Debug)]
enum FinancialStatusIndicator {
    Deficient,
    Delinquent,
    Bankrupt,
    Suspended,
    DeficientAndBankrupt,
    DeficientAndDelinquent,
    DeliquentAndBankrupt,
    DeficientDeliquentAndBankrupt,
    CreationsAndOrRedemptionsSuspendedForExchangeTradedProduct,
    Normal,
    Unknown,
}

impl FinancialStatusIndicator {
    
    fn new(symbol: &char) -> Self {
        match symbol {
            'D' => Self::Deficient,
            'E' => Self::Delinquent,
            'Q' => Self::Bankrupt,
            'S' => Self::Suspended,
            'G' => Self::DeficientAndBankrupt,
            'H' => Self::DeficientAndDelinquent,
            'J' => Self::DeliquentAndBankrupt,
            'K' => Self::DeficientDeliquentAndBankrupt,
            'C' => Self::CreationsAndOrRedemptionsSuspendedForExchangeTradedProduct,
            'N' => Self::Normal,
            _   => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RoundLotsOnly {
    OnlyRoundLot, // Nasdaq only accepts round lots
    NoOrderSizeRestriction, // Nasdaq does not have any order size restrictions for this security
}

impl RoundLotsOnly {
    fn new(symbol: &char) -> Self {
        match symbol {
            'Y' => Self::OnlyRoundLot,
            'N' => Self::NoOrderSizeRestriction,
            _   => unreachable!(),
        }
    }
}
#[derive(Debug)]
enum Authenticity {
    Production, // Prod
    Test, // Test
}

impl Authenticity {
    fn new(symbol: &char) -> Self {
        match symbol {
            'P' => Self::Production,
            'T' => Self::Test,
            _   => unreachable!(),
        }
    }
}
#[derive(Debug)]
enum ShortSaleThresholdIndicator {
    IssueIsRestricted, // Issue is restricted
    IssueIsNotRestricted, // Issue is not restricted
    Unknown, // Not available
}

impl ShortSaleThresholdIndicator {
    fn new(symbol: &char) -> Self {
        match symbol {
            'Y' => Self::IssueIsRestricted,
            'N' => Self::IssueIsNotRestricted,
            _   => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum IPOFlag {
    NewIPOSecurity, // listed instrument is set up as a new IPO security
    NotNewIPOSecurity, // listed instrument is not set up as a new IPO security
    Unknown, // Not available
}

impl IPOFlag {
    fn new(symbol: &char) -> Self {
        match symbol {
            'Y' => Self::NewIPOSecurity,
            'N' => Self::NotNewIPOSecurity,
            _   => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum LuldReferencePriceTier {
    Tier1, // tier 1 nms stock and select ETPs
    Tier2, // tier 2 nms stocks
    Unknown // not available
}

impl LuldReferencePriceTier {
    fn new(symbol: &i8) -> Self {
        match symbol {
            1 => Self::Tier1,
            2 => Self::Tier2,
            _ => Self::Unknown,
        }
    }
}
#[derive(Debug)]
enum ETPFlag {
    IsETP, //Instrument is an ETP
    IsNotETP, // Instrument is not an ETP
    Unknown // not available
}

impl ETPFlag {
    fn new(symbol: &char) -> Self {
        match symbol {
            'Y' => Self::IsETP,
            'N' => Self::IsNotETP,
            _   => Self::Unknown,
        }
    }
}
#[derive(Debug)]
enum ReverseIndicator {
    IsInverseETP, // Inverse ETP
    IsNotInverseETP, // Not an inverse ETP
}

impl ReverseIndicator {
    fn new(symbol: &char) -> Self {
        match symbol {
            'Y' => Self::IsInverseETP,
            'N' => Self::IsNotInverseETP,
            _   => unreachable!(),
        }
    }
}
#[derive(Debug)]
pub struct StockMessage {
    pub stock_locate:           u16,
    pub tracking_num:           u16,
    pub timestamp:              u64,
    pub stock_symbol:           String,
    pub market_cat:             MarketCategory,
    pub financial_status:       FinancialStatusIndicator,
    pub round_lot_size:         u32,
    pub round_lots_only:        RoundLotsOnly,
    pub issue_classification:   u8,
    pub issue_sub_type:         u16,
    pub authenticity:           Authenticity,
    pub short_sale_indicator:   ShortSaleThresholdIndicator,
    pub ipo_flag:               IPOFlag,
    pub price_tier:             LuldReferencePriceTier,
    pub etp_flag:               ETPFlag,
    pub ept_leverage_factor:    i32,
    pub inverse_indicator:      ReverseIndicator,
}



impl StockMessage {
    fn new(
        stock_locate: u16,
        tracking_num: u16,
        timestamp: u64,
        stock_symbol: String,
        market_cat: MarketCategory,
        financial_status: FinancialStatusIndicator,
        round_lot_size: u32,
        round_lots_only: RoundLotsOnly,
        issue_classification: u8,
        issue_sub_type: u16,
        authenticity: Authenticity,
        short_sale_indicator: ShortSaleThresholdIndicator,
        ipo_flag: IPOFlag,
        price_tier: LuldReferencePriceTier,
        etp_flag: ETPFlag,
        ept_leverage_factor: i32,
        inverse_indicator: ReverseIndicator
        ) -> Self {
        Self {
            stock_locate,
            tracking_num,
            timestamp,
            stock_symbol,
            market_cat,
            financial_status,
            round_lot_size,
            round_lots_only,
            issue_classification,
            issue_sub_type,
            authenticity,
            short_sale_indicator,
            ipo_flag,
            price_tier,
            etp_flag,
            ept_leverage_factor,
            inverse_indicator,
        }
    }

    pub fn format_summary(&self) -> String {
        let total_sec = self.timestamp / 1_000_000_000;
        let h = total_sec / 3600;
        let m = (total_sec % 3600) / 60;
        let s = total_sec % 60;

        format!(
            "[STOCK]  {:02}:{:02}:{:02} | Symbol: {:<5} | Market: {} | Locate: {}",
            h, m, s, self.stock_symbol, self.market_cat.as_str(), self.stock_locate
        )
    }
}









pub fn handle_stock_messages(buffer: &[u8]) -> StockMessage {
    let stock_locate = u16::from_be_bytes(buffer[1..3].try_into().unwrap());
    let tracking_num = u16::from_be_bytes(buffer[3..5].try_into().unwrap());

    let mut ts_bytes = [0u8;8];
    ts_bytes[2..8].copy_from_slice(&buffer[5..11]);
    let timestamp = u64::from_be_bytes(ts_bytes);

    let stock_symbol_raw = &buffer[11..19];
    let stock_symbol = String::from_utf8_lossy(stock_symbol_raw).trim().to_string();


    let market_category_symbol = u8::from_be_bytes(buffer[19..20].try_into().unwrap()) as char;
    let market_category = MarketCategory::new(&market_category_symbol);

    let financial_status_symbol = u8::from_be_bytes(buffer[20..21].try_into().unwrap()) as char;
    let financial_status_indicator = FinancialStatusIndicator::new(&financial_status_symbol);

    let round_lot_size = u32::from_be_bytes(buffer[21..25].try_into().unwrap());

    let round_lot_only_symbol = u8::from_be_bytes(buffer[25..26].try_into().unwrap()) as char;
    let round_lots_only = RoundLotsOnly::new(&round_lot_only_symbol);

    let issue_classification = u8::from_be_bytes(buffer[26..27].try_into().unwrap());

    let issue_sub_type = u16::from_be_bytes(buffer[27..29].try_into().unwrap());

    let authenticity_symbol = u8::from_be_bytes(buffer[29..30].try_into().unwrap()) as char;
    let authenticity = Authenticity::new(&authenticity_symbol);

    let short_sale_symbol = u8::from_be_bytes(buffer[30..31].try_into().unwrap()) as char;
    let short_sale_threshold_indicator = ShortSaleThresholdIndicator::new(&short_sale_symbol);

    let ipo_flag_symbol = u8::from_be_bytes(buffer[31..32].try_into().unwrap()) as char;
    let ipo_flag = IPOFlag::new(&ipo_flag_symbol);

    let luld_int = i8::from_be_bytes(buffer[32..33].try_into().unwrap());
    let luld_reference_price_tier = LuldReferencePriceTier::new(&luld_int);

    let etp_flag_symbol = u8::from_be_bytes(buffer[33..34].try_into().unwrap()) as char;
    let etp_flag = ETPFlag::new(&etp_flag_symbol);

    let etp_leverage_factor = i32::from_be_bytes(buffer[34..38].try_into().unwrap());

    let reverse_indicator_symbol = u8::from_be_bytes(buffer[38..39].try_into().unwrap()) as char;
    let reverse_indicator = ReverseIndicator::new(&reverse_indicator_symbol);


    StockMessage::new(
        stock_locate,
        tracking_num,
        timestamp,
        stock_symbol,
        market_category,
        financial_status_indicator,
        round_lot_size,
        round_lots_only,
        issue_classification,
        issue_sub_type,
        authenticity,
        short_sale_threshold_indicator,
        ipo_flag,
        luld_reference_price_tier,
        etp_flag,
        etp_leverage_factor,
        reverse_indicator,
        )




}

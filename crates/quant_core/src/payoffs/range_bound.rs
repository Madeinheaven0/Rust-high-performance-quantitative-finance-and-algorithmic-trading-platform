//! # Range-bound Option Strategies
//! Implementations for Iron Condor and Iron Butterfly payoff & PnL models.

use super::errors::PriceError;

/// Représente une position d'option individuelle dans une stratégie combinée.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptionLeg {
    pub strike: f64,
    pub premium: f64,
}

/// Iron Condor Strategy (Short)
///
/// Involves 4 strikes: K1 < K2 < K3 < K4
/// - Buy Put at K1
/// - Sell Put at K2
/// - Sell Call at K3
/// - Buy Call at K4
#[derive(Debug, Clone, PartialEq)]
pub struct IronCondor {
    pub put_buy: OptionLeg,   // K1
    pub put_sell: OptionLeg,  // K2
    pub call_sell: OptionLeg, // K3
    pub call_buy: OptionLeg,  // K4
}

impl IronCondor {
    pub fn new(
        strike_put_buy: f64,
        premium_put_buy: f64,
        strike_put_sell: f64,
        premium_put_sell: f64,
        strike_call_sell: f64,
        premium_call_sell: f64,
        strike_call_buy: f64,
        premium_call_buy: f64,
    ) -> Result<Self, PriceError> {
        // Validation des prix négatifs
        if strike_put_buy < 0. || strike_put_sell < 0. || strike_call_sell < 0. || strike_call_buy < 0. {
            return Err(PriceError::StrikePriceNegative);
        }

        if premium_put_buy <= 0. || premium_put_sell <= 0. || premium_call_sell <= 0. || premium_call_buy <= 0. {
            return Err(PriceError::PrimePriceError);
        }

        // Contrôle d'ordre strict des strikes : K1 < K2 < K3 < K4
        if !(strike_put_buy < strike_put_sell
            && strike_put_sell < strike_call_sell
            && strike_call_sell < strike_call_buy) {
            return Err(PriceError::StrikeConfigurationError(strike_put_buy, strike_call_buy));
        }

        Ok(Self {
            put_buy: OptionLeg { strike: strike_put_buy, premium: premium_put_buy },
            put_sell: OptionLeg { strike: strike_put_sell, premium: premium_put_sell },
            call_sell: OptionLeg { strike: strike_call_sell, premium: premium_call_sell },
            call_buy: OptionLeg { strike: strike_call_buy, premium: premium_call_buy },
        })
    }

    /// Prime nette reçue à l'ouverture (Net Credit)
    #[inline]
    pub fn net_premium(&self) -> f64 {
        (self.put_sell.premium + self.call_sell.premium)
            - (self.put_buy.premium + self.call_buy.premium)
    }

    /// Payoff brut à l'échéance pour un prix du sous-jacent `spot_price` donné.
    pub fn payoff_at(&self, spot_price: f64) -> f64 {
        let long_put = (self.put_buy.strike - spot_price).max(0.);
        let short_put = (self.put_sell.strike - spot_price).max(0.);
        let short_call = (spot_price - self.call_sell.strike).max(0.);
        let long_call = (spot_price - self.call_buy.strike).max(0.);

        long_put - short_put - short_call + long_call
    }

    /// Profit / Perte net à l'échéance (Payoff + Net Premium)
    #[inline]
    pub fn pnl_at(&self, spot_price: f64) -> f64 {
        self.payoff_at(spot_price) + self.net_premium()
    }

    /// Profit Maximum possible (Crédit net perçu)
    #[inline]
    pub fn max_profit(&self) -> f64 {
        self.net_premium()
    }

    /// Perte Maximale possible (Largeur du spread - Crédit net)
    pub fn max_loss(&self) -> f64 {
        let put_spread_width = self.put_sell.strike - self.put_buy.strike;
        let call_spread_width = self.call_buy.strike - self.call_sell.strike;
        let max_width = put_spread_width.max(call_spread_width);

        max_width - self.net_premium()
    }

    /// Points morts inférieur et supérieur (Break-even points)
    pub fn break_even_points(&self) -> (f64, f64) {
        let net_credit = self.net_premium();
        let lower_be = self.put_sell.strike - net_credit;
        let upper_be = self.call_sell.strike + net_credit;
        (lower_be, upper_be)
    }
}

/// Iron Butterfly Strategy (Short)
///
/// Involves 3 strikes: K1 < K2 (ATM Put & Call) < K3
/// - Buy Put at K1
/// - Sell Put at K2
/// - Sell Call at K2
/// - Buy Call at K3
#[derive(Debug, Clone, PartialEq)]
pub struct IronButterfly {
    pub put_buy: OptionLeg,   // K1
    pub atm_put_sell: OptionLeg,  // K2
    pub atm_call_sell: OptionLeg, // K2
    pub call_buy: OptionLeg,  // K3
}

impl IronButterfly {
    pub fn new(
        strike_put_buy: f64,
        premium_put_buy: f64,
        strike_atm: f64,
        premium_put_sell: f64,
        premium_call_sell: f64,
        strike_call_buy: f64,
        premium_call_buy: f64,
    ) -> Result<Self, PriceError> {
        if strike_put_buy < 0. || strike_atm < 0. || strike_call_buy < 0. {
            return Err(PriceError::StrikePriceNegative);
        }

        if premium_put_buy <= 0. || premium_put_sell <= 0. || premium_call_sell <= 0. || premium_call_buy <= 0. {
            return Err(PriceError::PrimePriceError);
        }

        if !(strike_put_buy < strike_atm && strike_atm < strike_call_buy) {
            return Err(PriceError::StrikeConfigurationError(strike_put_buy, strike_call_buy));
        }

        Ok(Self {
            put_buy: OptionLeg { strike: strike_put_buy, premium: premium_put_buy },
            atm_put_sell: OptionLeg { strike: strike_atm, premium: premium_put_sell },
            atm_call_sell: OptionLeg { strike: strike_atm, premium: premium_call_sell },
            call_buy: OptionLeg { strike: strike_call_buy, premium: premium_call_buy },
        })
    }

    #[inline]
    pub fn net_premium(&self) -> f64 {
        (self.atm_put_sell.premium + self.atm_call_sell.premium)
            - (self.put_buy.premium + self.call_buy.premium)
    }

    pub fn payoff_at(&self, spot_price: f64) -> f64 {
        let long_put = (self.put_buy.strike - spot_price).max(0.);
        let short_put = (self.atm_put_sell.strike - spot_price).max(0.);
        let short_call = (spot_price - self.atm_call_sell.strike).max(0.);
        let long_call = (spot_price - self.call_buy.strike).max(0.);

        long_put - short_put - short_call + long_call
    }

    #[inline]
    pub fn pnl_at(&self, spot_price: f64) -> f64 {
        self.payoff_at(spot_price) + self.net_premium()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // =========================================================================
    // FIXTURES / HELPERS
    // =========================================================================

    /// Fixture constructing a standard Short Iron Condor strategy.
    ///
    /// Configuration:
    /// - Strikes: K1 = 80.0 (Buy Put), K2 = 90.0 (Sell Put), K3 = 110.0 (Sell Call), K4 = 120.0 (Buy Call)
    /// - Premiums: Long Put = 1.0, Short Put = 3.0, Short Call = 3.0, Long Call = 1.0
    /// - Net Credit Received: (3.0 + 3.0) - (1.0 + 1.0) = 4.0
    fn setup_test_condor() -> IronCondor {
        IronCondor::new(
            80.0, 1.0,  // Long Put (K1)
            90.0, 3.0,  // Short Put (K2)
            110.0, 3.0, // Short Call (K3)
            120.0, 1.0, // Long Call (K4)
        )
            .expect("Iron Condor setup should be valid")
    }

    /// Fixture constructing a standard Short Iron Butterfly strategy.
    ///
    /// Configuration:
    /// - Strikes: K1 = 80.0 (Buy Put), K2 = 100.0 (Short ATM Put & Call), K3 = 120.0 (Buy Call)
    /// - Premiums: Long Put = 1.0, Short Put = 6.0, Short Call = 6.0, Long Call = 1.0
    /// - Net Credit Received: (6.0 + 6.0) - (1.0 + 1.0) = 10.0
    /// - Spread Width = 20.0 | Max Profit = 10.0 | Max Loss = 10.0
    /// - Break-even Points = 100 - 10 = 90.0 AND 100 + 10 = 110.0
    fn setup_test_butterfly() -> IronButterfly {
        IronButterfly::new(
            80.0, 1.0,   // Long Put (K1)
            100.0, 6.0,  // Short ATM Put (K2)
            6.0,         // Short ATM Call (K2)
            120.0, 1.0,  // Long Call (K3)
        )
            .expect("Iron Butterfly setup should be valid")
    }

    // =========================================================================
    // IRON CONDOR: Parametric Payoff & PnL Profile Test
    // =========================================================================

    #[rstest]
    // Spot < K1 (Below 80.0): Max Loss (-6.0)
    #[case(70.0, -10.0, -6.0)]
    // Spot = K1 (At 80.0): Max Loss (-6.0)
    #[case(80.0, -10.0, -6.0)]
    // Spot = Lower Break-even (86.0): Net PnL = 0.0
    #[case(86.0, -4.0, 0.0)]
    // Spot = K2 (At 90.0): Gross Payoff = 0.0, Net PnL = Max Profit (+4.0)
    #[case(90.0, 0.0, 4.0)]
    // Spot inside neutral zone (At 100.0): Gross Payoff = 0.0, Net PnL = Max Profit (+4.0)
    #[case(100.0, 0.0, 4.0)]
    // Spot = K3 (At 110.0): Gross Payoff = 0.0, Net PnL = Max Profit (+4.0)
    #[case(110.0, 0.0, 4.0)]
    // Spot = Upper Break-even (114.0): Net PnL = 0.0
    #[case(114.0, -4.0, 0.0)]
    // Spot = K4 (At 120.0): Max Loss (-6.0)
    #[case(120.0, -10.0, -6.0)]
    // Spot > K4 (Above 120.0): Max Loss (-6.0)
    #[case(130.0, -10.0, -6.0)]
    fn test_iron_condor_payoff_and_pnl_profile(
        #[case] spot_price: f64,
        #[case] expected_payoff: f64,
        #[case] expected_pnl: f64,
    ) {
        let condor = setup_test_condor();

        assert_eq!(
            condor.payoff_at(spot_price),
            expected_payoff,
            "Iron Condor payoff calculation failed at spot price {}", spot_price
        );
        assert_eq!(
            condor.pnl_at(spot_price),
            expected_pnl,
            "Iron Condor PnL calculation failed at spot price {}", spot_price
        );
    }

    // =========================================================================
    // IRON BUTTERFLY: Parametric Payoff & PnL Profile Test
    // =========================================================================

    #[rstest]
    // Spot < K1 (Below 80.0): Max Loss (-10.0)
    #[case(70.0, -20.0, -10.0)]
    // Spot = K1 (At 80.0): Max Loss (-10.0)
    #[case(80.0, -20.0, -10.0)]
    // Spot = Lower Break-even (90.0): Net PnL = 0.0
    #[case(90.0, -10.0, 0.0)]
    // Spot = K2 Peak ATM (At 100.0): Gross Payoff = 0.0, Net PnL = Max Profit (+10.0)
    #[case(100.0, 0.0, 10.0)]
    // Spot = Upper Break-even (110.0): Net PnL = 0.0
    #[case(110.0, -10.0, 0.0)]
    // Spot = K3 (At 120.0): Max Loss (-10.0)
    #[case(120.0, -20.0, -10.0)]
    // Spot > K3 (Above 120.0): Max Loss (-10.0)
    #[case(130.0, -20.0, -10.0)]
    fn test_iron_butterfly_payoff_and_pnl_profile(
        #[case] spot_price: f64,
        #[case] expected_payoff: f64,
        #[case] expected_pnl: f64,
    ) {
        let butterfly = setup_test_butterfly();

        assert_eq!(
            butterfly.payoff_at(spot_price),
            expected_payoff,
            "Iron Butterfly payoff calculation failed at spot price {}", spot_price
        );
        assert_eq!(
            butterfly.pnl_at(spot_price),
            expected_pnl,
            "Iron Butterfly PnL calculation failed at spot price {}", spot_price
        );
    }

    // =========================================================================
    // RISK METRICS & VALIDATION TESTS
    // =========================================================================

    #[test]
    fn test_iron_condor_risk_metrics() {
        let condor = setup_test_condor();

        assert_eq!(condor.net_premium(), 4.0);
        assert_eq!(condor.max_profit(), 4.0);
        assert_eq!(condor.max_loss(), 6.0);

        let (lower_be, upper_be) = condor.break_even_points();
        assert_eq!(lower_be, 86.0);
        assert_eq!(upper_be, 114.0);
    }

    #[test]
    fn test_validation_errors() {
        // Misordered strikes validation: K1 > K2
        assert!(matches!(
            IronCondor::new(95.0, 1.0, 90.0, 3.0, 110.0, 3.0, 120.0, 1.0),
            Err(PriceError::StrikeConfigurationError(_, _))
        ));

        // Negative strike price validation
        assert!(matches!(
            IronCondor::new(-80.0, 1.0, 90.0, 3.0, 110.0, 3.0, 120.0, 1.0),
            Err(PriceError::StrikePriceNegative)
        ));

        // Non-positive option premium validation
        assert!(matches!(
            IronCondor::new(80.0, 0.0, 90.0, 3.0, 110.0, 3.0, 120.0, 1.0),
            Err(PriceError::PrimePriceError)
        ));
    }
}

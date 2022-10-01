//! # Wallet

use rust_decimal::Decimal;

/// A wallet represents the quantity of an asset hold by the investor
#[derive(Debug, Default)]
pub struct Wallet {
    blocks: Vec<Block>,
}

impl Wallet {
    /// Buy (add) a block to the wallet
    pub fn buy(&mut self, amount_asset: Decimal, amount_fiat: Decimal) {
        self.blocks.push(Block::new(amount_asset, amount_fiat));
    }

    /// Returns the total fiat amount of the wallet
    pub fn amount_fiat(&self) -> Decimal {
        self.blocks.iter().map(|x| x.amount_fiat).sum()
    }

    /// Returns the total asset amount of the wallet
    pub fn amount_asset(&self) -> Decimal {
        self.blocks.iter().map(|x| x.amount_asset).sum()
    }

    /// Sell an asset amount hold in the wallet.
    /// Returns the FIAT amount sold (NOTE: refers to the buy price, not to the sell price)
    /// Returns error if `amount_asset > self.amount_asset()`
    pub fn sell(&mut self, amount_asset: Decimal) -> anyhow::Result<Decimal> {
        debug!(
            "spending {} in wallet (current amount: {})",
            amount_asset,
            self.amount_asset()
        );
        if amount_asset > self.amount_asset() {
            anyhow::bail!(
                "cannot spend {} of asset in the wallet. Not enough assets: {}",
                amount_asset,
                self.amount_asset()
            );
        }
        // reset wallet
        let blocks = self.blocks.clone();
        self.blocks = Vec::with_capacity(blocks.len());
        let mut remaining_assets_to_sell = amount_asset;
        let mut amount_fiat = Decimal::ZERO;
        // sell blocks
        for block in blocks.into_iter() {
            if remaining_assets_to_sell.is_zero() {
                debug!("all required blocks have already been sold; just pushing block ({}; € {}) to wallet", block.amount_asset, block.amount_fiat);
                self.blocks.push(block);
            } else if block.amount_asset <= remaining_assets_to_sell {
                // sell entire block
                debug!(
                    "sold entire block ({}; € {})",
                    block.amount_asset, block.amount_fiat
                );
                amount_fiat += block.amount_fiat;
                remaining_assets_to_sell -= block.amount_asset;
            } else {
                // remaining_assets_to_sell < block.amount_asset
                let (unsold_block, sold_block) =
                    self.sell_partial_block(block, remaining_assets_to_sell);
                debug!(
                    "a fraction of block has been sold. New blocks ({}; € {}),({}; € {})",
                    unsold_block.amount_asset,
                    unsold_block.amount_fiat,
                    sold_block.amount_asset,
                    sold_block.amount_fiat
                );
                // zero remaining_assets_to_sell
                remaining_assets_to_sell = Decimal::ZERO;
                // push unsold block to wallet
                self.blocks.push(unsold_block);
                // increment amount fiat by sold block value
                amount_fiat += sold_block.amount_fiat;
            }
        }
        debug!(
            "sold {} assets, which is worth € {}",
            amount_asset, amount_fiat
        );
        Ok(amount_fiat)
    }

    /// Perform a stock split on the wallet.
    /// All the currents blocks are replaced by a new block, which is so composed:
    ///
    /// - the fiat amount is equal to the sum of all the current block's fiat amount
    /// - the asset amount is `amount_asset` + `self.amount_asset`
    pub fn stock_split(&mut self, amount_asset: Decimal) {
        let amount_fiat = self.amount_fiat();
        let new_amount_asset = self.amount_asset() + amount_asset;
        debug!("stock split: ({}; € {})", new_amount_asset, amount_fiat);
        self.blocks = vec![Block::new(new_amount_asset, amount_fiat)];
    }

    /// Sell partial block. Starting from the amount_asset; which must be LESS THAN block.amount_asset, returns two blocks.
    /// The first one is the UNSOLD block, while the second one, is the block to sell
    fn sell_partial_block(&self, mut block: Block, amount_asset: Decimal) -> (Block, Block) {
        assert!(amount_asset < block.amount_asset);
        let fraction = block.sell_fraction(amount_asset);
        (block, fraction)
    }
}

/// A wallet block represents a spendible amount of a certain asset
#[derive(Debug, Copy, Clone)]
pub struct Block {
    /// The quantity of the block
    amount_asset: Decimal,
    /// The FIAT value of this block
    amount_fiat: Decimal,
}

impl Block {
    /// Instantiate a new Block
    pub fn new(amount_asset: Decimal, amount_fiat: Decimal) -> Self {
        Self {
            amount_asset,
            amount_fiat,
        }
    }

    /// Sell a fraction of the block. The amount is subtracted from this block and the fraction is returned
    pub fn sell_fraction(&mut self, amount_asset: Decimal) -> Self {
        // calc new amount_fiat => self.amount_fiat : self.amount_asset = x : amount_asset
        let fraction_amount_fiat = (self.amount_fiat * amount_asset) / self.amount_asset;
        // change this block's values
        self.amount_fiat -= fraction_amount_fiat;
        self.amount_asset -= amount_asset;
        // return fraction
        Block::new(amount_asset, fraction_amount_fiat)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_block() {
        let block = Block::new(dec!(2.0), dec!(186.32));
        assert_eq!(block.amount_asset, dec!(2.0));
        assert_eq!(block.amount_fiat, dec!(186.32));
    }

    #[test]
    fn should_sell_block_fraction() {
        let mut block = Block::new(dec!(2.25), dec!(186.32));
        let fraction = block.sell_fraction(dec!(0.75));
        assert_eq!(block.amount_asset, dec!(1.5));
        assert_eq!(block.amount_fiat.round_dp(2), dec!(124.21));
        assert_eq!(fraction.amount_asset, dec!(0.75));
        assert_eq!(fraction.amount_fiat.round_dp(2), dec!(62.11));
    }

    #[test]
    fn should_init_wallet() {
        let mut wallet = Wallet::default();
        assert!(wallet.blocks.is_empty());
        wallet.buy(dec!(2.0), dec!(186.32));
        wallet.buy(dec!(0.5), dec!(68.78));
        wallet.buy(dec!(1.25), dec!(104.32));
        assert_eq!(wallet.blocks.len(), 3);
        assert_eq!(wallet.amount_asset(), dec!(3.75));
        assert_eq!(wallet.amount_fiat(), dec!(359.42));
    }

    #[test]
    fn should_sell_wallet_blocks_entire_block() {
        let mut wallet = Wallet::default();
        wallet.buy(dec!(2.0), dec!(186.32));
        wallet.buy(dec!(0.5), dec!(68.78));
        wallet.buy(dec!(1.25), dec!(104.32));
        // sell
        assert_eq!(wallet.sell(dec!(2.0)).unwrap(), dec!(186.32));
        assert_eq!(wallet.amount_asset(), dec!(1.75));
        assert_eq!(wallet.amount_fiat(), dec!(173.10));
    }

    #[test]
    fn should_sell_wallet_blocks_entire_wallet() {
        let mut wallet = Wallet::default();
        wallet.buy(dec!(2.0), dec!(186.32));
        wallet.buy(dec!(0.5), dec!(68.78));
        wallet.buy(dec!(1.25), dec!(104.32));
        // sell
        assert_eq!(wallet.sell(dec!(3.75)).unwrap(), dec!(359.42));
        assert_eq!(wallet.amount_asset(), dec!(0));
        assert_eq!(wallet.amount_fiat(), dec!(0));
    }

    #[test]
    fn should_sell_wallet_blocks_partial() {
        let mut wallet = Wallet::default();
        wallet.buy(dec!(2.0), dec!(186.32));
        wallet.buy(dec!(0.5), dec!(68.78));
        wallet.buy(dec!(1.25), dec!(104.32));
        // sell
        assert_eq!(wallet.sell(dec!(2.40)).unwrap(), dec!(241.344)); // 55.024
        assert_eq!(wallet.amount_asset(), dec!(1.35));
        assert_eq!(wallet.amount_fiat(), dec!(118.076));
    }

    #[test]
    fn should_fail_selling_wallet_blocks_if_more_than_balance() {
        let mut wallet = Wallet::default();
        wallet.buy(dec!(2.0), dec!(186.32));
        wallet.buy(dec!(0.5), dec!(68.78));
        assert!(wallet.sell(dec!(5.0)).is_err());
    }

    #[test]
    fn should_perform_stock_split() {
        let mut wallet = Wallet::default();
        wallet.buy(dec!(0.025), dec!(186.32));
        wallet.buy(dec!(0.01), dec!(68.78));
        wallet.buy(dec!(0.015), dec!(104.32));

        wallet.stock_split(dec!(1.34));
        assert_eq!(wallet.amount_asset(), dec!(1.39));
        assert_eq!(wallet.amount_fiat(), dec!(359.42));
    }
}

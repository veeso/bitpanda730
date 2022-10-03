//! # Ticker whitelist
//!
//! This module contains the whitelist for taxes for tickers

const CHINESE_GOVERNMENT_BONDS: &str = "CHINABOND";
const EUROZONE_GOVERNMENT_BONDS: &str = "EUROGOV";
const JAPANESE_GOVERNMENT_BONDS: &str = "JAPGOVIES";
const US_GOVERNMENT_BONDS: &str = "USGOVIES";

pub struct TickerWhitelist;

impl TickerWhitelist {
    /// Returns whether provided ticker is whitelisted according to the Italian rules:
    ///
    /// - BONDs whitelist:
    ///
    /// - Albania
    /// - Alderney
    /// - Algeria
    /// - Anguilla
    /// - Arabia Saudita
    /// - Argentina
    /// - Armenia
    /// - Aruba
    /// - Australia
    /// - Austria
    /// - Azerbajan
    /// - Bangladesh
    /// - Belgio
    /// - Belize
    /// - Bermuda
    /// - Bielorussia
    /// - Bosnia Ersegovina
    /// - Brasile
    /// - Bulgaria
    /// - Camerun
    /// - Canada
    /// - Cina
    /// - Cipro
    /// - Colombia
    /// - Congo
    /// - Corea del Sud
    /// - Costa d'Avorio
    /// - Costa Rica
    /// - Croazia
    /// - Curacao
    /// - Danimarca
    /// - Ecuador
    /// - Egitto
    /// - Emirati Arabi Uniti
    /// - Estonia
    /// - Etiopia
    /// - Federazione Russa
    /// - Filippine
    /// - Finlandia
    /// - Francia
    /// - Georgia
    /// - Germania
    /// - Ghana
    /// - Giappone
    /// - Gibilterra
    /// - Giordania
    /// - Grecia
    /// - Groenlandia
    /// - Guernsey
    /// - Herm
    /// - Hong Kong
    /// - India
    /// - Indonesia
    /// - Irlanda
    /// - Islanda
    /// - Isola di Man
    /// - Isole Cayman
    /// - Isole Cook
    /// - Isole Faroe
    /// - Isole Turks e Caicos
    /// - Isole Vergini Britanniche
    /// - Israele
    /// - Jersey
    /// - Kazakistan
    /// - Kirghistan
    /// - Kuwait
    /// - Lettonia
    /// - Libano
    /// - Liechtenstein
    /// - Lituania
    /// - Lussemburgo
    /// - Macedonia
    /// - Malaysia
    /// - Malta
    /// - Marocco
    /// - Mauritius
    /// - Messico
    /// - Moldova
    /// - Montenegro
    /// - Montserrat
    /// - Mozambico
    /// - Nigeria
    /// - Norvegia
    /// - Nuova Zelanda
    /// - Oman
    /// - Paesi Bassi
    /// - Pakistan
    /// - Polonia
    /// - Portogallo
    /// - Qatar
    /// - Regno Unito
    /// - Repubblica Ceca
    /// - Repubblica Slovacca
    /// - Romania
    /// - San Marino
    /// - Senegal
    /// - Serbia
    /// - Seychelles
    /// - Singapore
    /// - Sint Maarten
    /// - Siria
    /// - Slovenia
    /// - Spagna
    /// - Sri Lanka
    /// - Stati Uniti
    /// - Sud Africa
    /// - Svezia
    /// - Svizzera
    /// - Tagikistan
    /// - Taiwan
    /// - Tanzania
    /// - Thailandia
    /// - Trinidad e Tobago
    /// - Tunisia
    /// - Turchia
    /// - Uganda
    /// - Turkmenistan
    /// - Ucraina
    /// - Ungheria
    /// - Uzbekistan
    /// - Venezuela
    /// - Vietnam
    /// - Zambia
    pub fn is_whitelisted(ticker: &str) -> bool {
        matches!(
            ticker,
            CHINESE_GOVERNMENT_BONDS
                | EUROZONE_GOVERNMENT_BONDS
                | JAPANESE_GOVERNMENT_BONDS
                | US_GOVERNMENT_BONDS
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_ticker_is_whitelisted() {
        assert!(TickerWhitelist::is_whitelisted(CHINESE_GOVERNMENT_BONDS));
        assert!(TickerWhitelist::is_whitelisted(EUROZONE_GOVERNMENT_BONDS));
        assert!(TickerWhitelist::is_whitelisted(JAPANESE_GOVERNMENT_BONDS));
        assert!(TickerWhitelist::is_whitelisted(US_GOVERNMENT_BONDS));
        assert_eq!(TickerWhitelist::is_whitelisted("AMZN"), false);
    }
}


#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use crate::modules::technical::indicators::{
        calculate_basic_stats, get_latest_quote, get_ohlc_data, get_ohlc_data_range, search_ticker, OhlcData, OhlcVecExt
    };

    // Fonction utilitaire pour créer des données de test
    fn create_test_data() -> Vec<OhlcData> {
        vec![
            OhlcData {
                timestamp: 1704067200, // 2024-01-01
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 1000000,
            },
            OhlcData {
                timestamp: 1704153600, // 2024-01-02
                open: 102.0,
                high: 107.0,
                low: 100.0,
                close: 105.0,
                volume: 1200000,
            },
            OhlcData {
                timestamp: 1704240000, // 2024-01-03
                open: 105.0,
                high: 110.0,
                low: 103.0,
                close: 108.0,
                volume: 900000,
            },
            OhlcData {
                timestamp: 1704326400, // 2024-01-04
                open: 108.0,
                high: 112.0,
                low: 106.0,
                close: 110.0,
                volume: 1100000,
            },
            OhlcData {
                timestamp: 1704412800, // 2024-01-05
                open: 110.0,
                high: 115.0,
                low: 108.0,
                close: 112.0,
                volume: 1300000,
            },
        ]
    }


    // Tests pour les fonctions de filtrage
    #[test]
    fn test_filter_last_days() {
        let test_data = create_test_data();
        
        // Test avec moins d'éléments que demandé
        let filtered = test_data.filter_last_days(10);
        assert_eq!(filtered.len(), 5);
        
        // Test avec plus d'éléments que demandé
        let filtered = test_data.filter_last_days(3);
        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0].timestamp, 1704240000); // 2024-01-03
        assert_eq!(filtered[1].timestamp, 1704326400); // 2024-01-04
        assert_eq!(filtered[2].timestamp, 1704412800); // 2024-01-05
        
        // Test avec 0 éléments
        let filtered = test_data.filter_last_days(0);
        assert_eq!(filtered.len(), 0);
    }


    // Tests pour les statistiques
    #[test]
    fn test_calculate_basic_stats() {
        let test_data = create_test_data();
        let stats = calculate_basic_stats(&test_data);
        
        assert!(stats.is_some());
        
        if let Some(stats) = stats {
            assert_eq!(stats.min_price, 102.0);
            assert_eq!(stats.max_price, 112.0);
            assert_eq!(stats.avg_price, 107.4); // (102+105+108+110+112)/5
            assert_eq!(stats.total_volume, 5500000);
            assert_eq!(stats.avg_volume, 1100000);
            assert_eq!(stats.data_points, 5);
        }
    }

    #[test]
    fn test_calculate_basic_stats_empty() {
        let empty_data: Vec<OhlcData> = vec![];
        let stats = calculate_basic_stats(&empty_data);
        assert!(stats.is_none());
    }

    #[test]
    fn test_calculate_basic_stats_single_element() {
        let single_data = vec![OhlcData {
            timestamp: 1704067200,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000000,
        }];
        
        let stats = calculate_basic_stats(&single_data);
        assert!(stats.is_some());
        
        if let Some(stats) = stats {
            assert_eq!(stats.min_price, 102.0);
            assert_eq!(stats.max_price, 102.0);
            assert_eq!(stats.avg_price, 102.0);
            assert_eq!(stats.total_volume, 1000000);
            assert_eq!(stats.avg_volume, 1000000);
            assert_eq!(stats.data_points, 1);
        }
    }

    // Tests d'intégration avec l'API (tests lents)
    #[tokio::test]
    #[ignore] // Utilisez `cargo test -- --ignored` pour exécuter ces tests
    async fn test_get_ohlc_data_integration() {
        let result = get_ohlc_data("AAPL", "1d", "5d").await;
        assert!(result.is_ok(), "Failed to fetch OHLC data: {:?}", result.err());
        
        if let Ok(data) = result {
            assert!(!data.is_empty(), "No data returned");
            
            // Vérifier que les données sont bien triées par timestamp
            for i in 1..data.len() {
                assert!(
                    data[i].timestamp >= data[i - 1].timestamp,
                    "Data not sorted by timestamp"
                );
            }
            
            // Vérifier que les données ont des valeurs raisonnables
            for item in &data {
                assert!(item.open > 0.0, "Invalid open price");
                assert!(item.high > 0.0, "Invalid high price");
                assert!(item.low > 0.0, "Invalid low price");
                assert!(item.close > 0.0, "Invalid close price");
                assert!(item.high >= item.low, "High should be >= low");
                assert!(item.volume > 0, "Invalid volume");
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_latest_quote_integration() {
        let result = get_latest_quote("AAPL").await;
        assert!(result.is_ok(), "Failed to fetch latest quote: {:?}", result.err());
        
        if let Ok(quote) = result {
            assert!(quote.open > 0.0);
            assert!(quote.high > 0.0);
            assert!(quote.low > 0.0);
            assert!(quote.close > 0.0);
            assert!(quote.volume > 0);
            assert!(quote.high >= quote.low);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_search_ticker_integration() {
        let result = search_ticker("Apple").await;
        assert!(result.is_ok(), "Failed to search ticker: {:?}", result.err());
        
        if let Ok(tickers) = result {
            assert!(!tickers.is_empty(), "No tickers found");
            assert!(tickers.contains(&"AAPL".to_string()), "AAPL not found in search");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_ohlc_data_range_integration() {
        let start = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
            .unwrap()
            .with_timezone(&Utc);
        
        let result = get_ohlc_data_range("AAPL", start, end).await;
        assert!(result.is_ok(), "Failed to fetch range data: {:?}", result.err());
        
        if let Ok(data) = result {
            // Vérifier que toutes les données sont dans la plage demandée
            for item in &data {
                assert!(
                    item.timestamp >= start.timestamp() && item.timestamp <= end.timestamp(),
                    "Data outside requested range"
                );
            }
        }
    }

    // Tests d'erreur
    #[tokio::test]
    #[ignore]
    async fn test_invalid_symbol() {
        let result = get_ohlc_data("INVALID_SYMBOL_XYZ", "1d", "1mo").await;
        // Selon l'API, cela pourrait soit échouer soit retourner des données vides
        if let Ok(data) = result {
            // Si l'API ne retourne pas d'erreur, les données devraient être vides
            assert!(data.is_empty(), "Expected empty data for invalid symbol");
        }
        // Sinon, c'est OK si ça retourne une erreur
    }

    // Tests de performance et de robustesse
    #[test]
    fn test_large_dataset_filtering() {
        // Créer un grand dataset pour tester les performances
        let mut large_data = Vec::new();
        for i in 0..10000 {
            large_data.push(OhlcData {
                timestamp: 1704067200 + (i * 86400), // Un jour par élément
                open: 100.0 + (i as f64 * 0.1),
                high: 105.0 + (i as f64 * 0.1),
                low: 95.0 + (i as f64 * 0.1),
                close: 102.0 + (i as f64 * 0.1),
                volume: 1_000_000u64 + (i as u64 * 1_000),

            });
        }
        
        // Test de filtrage sur un grand dataset
        let filtered = large_data.filter_last_days( 100);
        assert_eq!(filtered.len(), 100);
        
        // Test de statistiques sur un grand dataset
        let stats = calculate_basic_stats(&large_data);
        assert!(stats.is_some());
        
        if let Some(stats) = stats {
            assert_eq!(stats.data_points, 10000);
            assert!(stats.min_price < stats.max_price);
        }
    }

    #[test]
    fn test_basic_stats_print() {
        let test_data = create_test_data();
        let stats = calculate_basic_stats(&test_data).unwrap();
        
        // Test que print_stats ne panique pas
        stats.print_stats();
    }

    // Test des cas limites
    #[test]
    fn test_edge_cases() {
        // Données avec des prix identiques
        let identical_prices = vec![
            OhlcData {
                timestamp: 1704067200,
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
                volume: 1000,
            },
            OhlcData {
                timestamp: 1704153600,
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
                volume: 1000,
            },
        ];
        
        let stats = calculate_basic_stats(&identical_prices);
        assert!(stats.is_some());
        
        if let Some(stats) = stats {
            assert_eq!(stats.min_price, 100.0);
            assert_eq!(stats.max_price, 100.0);
            assert_eq!(stats.avg_price, 100.0);
        }
    }
}
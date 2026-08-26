//! Routing compaction for venue_type/provider/bridge metadata
//!
//! Provides lossless round-trip conversion between compact and expanded edge representations.
//! Bridge venue_type cannot be laundered into sdex or amm.

use serde::{Deserialize, Serialize};

/// Compact edge representation for storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompactEdge {
    pub from: String,
    pub to: String,
    pub venue_type: u8,
    pub venue_ref: String,
    pub liquidity: i128,
    pub price_bits: u64,
    pub fee_bps: u16,
}

/// Expanded edge representation for routing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpandedEdge {
    pub from: String,
    pub to: String,
    pub venue_type: String,
    pub venue_ref: String,
    pub liquidity: i128,
    pub price: f64,
    pub fee_bps: u32,
}

/// Venue type constants
pub const VENUE_TYPE_SDEX: u8 = 0;
pub const VENUE_TYPE_AMM: u8 = 1;
pub const VENUE_TYPE_BRIDGE: u8 = 2;

/// Convert venue type string to compact u8
pub fn venue_type_to_u8(venue_type: &str) -> Option<u8> {
    match venue_type {
        "sdex" => Some(VENUE_TYPE_SDEX),
        "amm" => Some(VENUE_TYPE_AMM),
        "bridge" => Some(VENUE_TYPE_BRIDGE),
        _ => None,
    }
}

/// Convert compact u8 to venue type string
pub fn u8_to_venue_type(venue_type: u8) -> Option<&'static str> {
    match venue_type {
        VENUE_TYPE_SDEX => Some("sdex"),
        VENUE_TYPE_AMM => Some("amm"),
        VENUE_TYPE_BRIDGE => Some("bridge"),
        _ => None,
    }
}

/// Compact an expanded edge into storage format
pub fn from_edges(edges: &[ExpandedEdge]) -> Vec<CompactEdge> {
    edges
        .iter()
        .filter_map(|edge| {
            let venue_type_u8 = venue_type_to_u8(&edge.venue_type)?;
            let price_bits = edge.price.to_bits();
            Some(CompactEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                venue_type: venue_type_u8,
                venue_ref: edge.venue_ref.clone(),
                liquidity: edge.liquidity,
                price_bits,
                fee_bps: edge.fee_bps as u16,
            })
        })
        .collect()
}

/// Expand a compact edge into routing format
pub fn to_edges(edges: &[CompactEdge]) -> Vec<ExpandedEdge> {
    edges
        .iter()
        .filter_map(|edge| {
            let venue_type_str = u8_to_venue_type(edge.venue_type)?;
            let price = f64::from_bits(edge.price_bits);
            Some(ExpandedEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                venue_type: venue_type_str.to_string(),
                venue_ref: edge.venue_ref.clone(),
                liquidity: edge.liquidity,
                price,
                fee_bps: edge.fee_bps as u32,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sdex_edge() -> ExpandedEdge {
        ExpandedEdge {
            from: "native".to_string(),
            to: "USDC:issuer".to_string(),
            venue_type: "sdex".to_string(),
            venue_ref: "sdex:1001".to_string(),
            liquidity: 10_000_000,
            price: 0.1,
            fee_bps: 0,
        }
    }

    fn amm_edge() -> ExpandedEdge {
        ExpandedEdge {
            from: "native".to_string(),
            to: "USDC:issuer".to_string(),
            venue_type: "amm".to_string(),
            venue_ref: "CAMMPOOL1".to_string(),
            liquidity: 5_000_000,
            price: 0.099,
            fee_bps: 30,
        }
    }

    fn bridge_edge() -> ExpandedEdge {
        ExpandedEdge {
            from: "USDC:issuer".to_string(),
            to: "USDC:eth_issuer".to_string(),
            venue_type: "bridge".to_string(),
            venue_ref: "bridge:cctp".to_string(),
            liquidity: 1_000_000_000,
            price: 1.0,
            fee_bps: 10,
        }
    }

    #[test]
    fn test_sdex_round_trip() {
        let original = vec![sdex_edge()];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].venue_type, "sdex");
        assert_eq!(expanded[0].from, original[0].from);
        assert_eq!(expanded[0].to, original[0].to);
        assert_eq!(expanded[0].liquidity, original[0].liquidity);
    }

    #[test]
    fn test_amm_round_trip() {
        let original = vec![amm_edge()];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].venue_type, "amm");
        assert_eq!(expanded[0].fee_bps, 30);
    }

    #[test]
    fn test_bridge_round_trip() {
        let original = vec![bridge_edge()];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].venue_type, "bridge");
        assert_eq!(expanded[0].venue_ref, "bridge:cctp");
    }

    #[test]
    fn test_bridge_not_laundered_to_sdex() {
        let original = vec![bridge_edge()];
        let compact = from_edges(&original);
        
        // Verify compact edge has bridge type
        assert_eq!(compact[0].venue_type, VENUE_TYPE_BRIDGE);
        
        // Expand and verify it's still bridge
        let expanded = to_edges(&compact);
        assert_eq!(expanded[0].venue_type, "bridge");
        assert_ne!(expanded[0].venue_type, "sdex");
        assert_ne!(expanded[0].venue_type, "amm");
    }

    #[test]
    fn test_bridge_not_laundered_to_amm() {
        let original = vec![bridge_edge()];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert_eq!(expanded[0].venue_type, "bridge");
        assert_ne!(expanded[0].venue_type, "amm");
    }

    #[test]
    fn test_mixed_venue_types_round_trip() {
        let original = vec![sdex_edge(), amm_edge(), bridge_edge()];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert_eq!(expanded.len(), 3);
        assert_eq!(expanded[0].venue_type, "sdex");
        assert_eq!(expanded[1].venue_type, "amm");
        assert_eq!(expanded[2].venue_type, "bridge");
    }

    #[test]
    fn test_price_precision_lossy() {
        let original = vec![ExpandedEdge {
            from: "native".to_string(),
            to: "USDC:issuer".to_string(),
            venue_type: "sdex".to_string(),
            venue_ref: "sdex:1001".to_string(),
            liquidity: 10_000_000,
            price: 0.123456789,
            fee_bps: 0,
        }];
        
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        // f64 round-trip is lossy but should be close
        assert!((expanded[0].price - 0.123456789).abs() < 1e-10);
    }

    #[test]
    fn test_venue_type_conversion() {
        assert_eq!(venue_type_to_u8("sdex"), Some(VENUE_TYPE_SDEX));
        assert_eq!(venue_type_to_u8("amm"), Some(VENUE_TYPE_AMM));
        assert_eq!(venue_type_to_u8("bridge"), Some(VENUE_TYPE_BRIDGE));
        assert_eq!(venue_type_to_u8("invalid"), None);
        
        assert_eq!(u8_to_venue_type(VENUE_TYPE_SDEX), Some("sdex"));
        assert_eq!(u8_to_venue_type(VENUE_TYPE_AMM), Some("amm"));
        assert_eq!(u8_to_venue_type(VENUE_TYPE_BRIDGE), Some("bridge"));
        assert_eq!(u8_to_venue_type(99), None);
    }

    #[test]
    fn test_empty_edges() {
        let original: Vec<ExpandedEdge> = vec![];
        let compact = from_edges(&original);
        let expanded = to_edges(&compact);
        
        assert!(compact.is_empty());
        assert!(expanded.is_empty());
    }
}

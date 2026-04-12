/// Registry entry mapping a short resource name to its API path prefix.
pub struct ServiceEntry {
    pub name: &'static str,
    pub path_prefix: &'static str,
    pub description: &'static str,
}

/// All known Reverb API resources.
pub const SERVICES: &[ServiceEntry] = &[
    ServiceEntry {
        name: "listings",
        path_prefix: "listings",
        description: "Manage your Reverb listings",
    },
    // ServiceEntry {
    //     name: "orders",
    //     path_prefix: "orders",
    //     description: "View and manage orders",
    // },
    // ServiceEntry {
    //     name: "conversations",
    //     path_prefix: "conversations",
    //     description: "Buyer/seller messaging",
    // },
    // ServiceEntry {
    //     name: "shop",
    //     path_prefix: "shop",
    //     description: "Shop profile and settings",
    // },
    // ServiceEntry {
    //     name: "categories",
    //     path_prefix: "categories",
    //     description: "Browse Reverb categories",
    // },
    // ServiceEntry {
    //     name: "handpicked",
    //     path_prefix: "handpicked",
    //     description: "Curated handpicked collections",
    // },
    // ServiceEntry {
    //     name: "priceguide",
    //     path_prefix: "priceguide",
    //     description: "Price guide for instruments",
    // },
    // ServiceEntry {
    //     name: "shipping",
    //     path_prefix: "shipping",
    //     description: "Shipping profiles and rates",
    // },
    // ServiceEntry {
    //     name: "feedback",
    //     path_prefix: "feedback",
    //     description: "Seller and buyer feedback",
    // },
    // ServiceEntry {
    //     name: "webhooks",
    //     path_prefix: "webhooks",
    //     description: "Manage webhook subscriptions",
    // },
];

pub fn find_service(name: &str) -> Option<&'static ServiceEntry> {
    SERVICES.iter().find(|s| s.name == name)
}

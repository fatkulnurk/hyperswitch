//! Commonly used constants

use std::collections::HashSet;

/// Number of characters in a generated ID
pub const ID_LENGTH: usize = 20;

/// Characters to use for generating NanoID
pub(crate) const ALPHABETS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// TTL for token
pub const TOKEN_TTL: i64 = 900;

///an example of the frm_configs json
pub static FRM_CONFIGS_EG: &str = r#"
[{"gateway":"stripe","payment_methods":[{"payment_method":"card","payment_method_types":[{"payment_method_type":"credit","card_networks":["Visa"],"flow":"pre","action":"cancel_txn"},{"payment_method_type":"debit","card_networks":["Visa"],"flow":"pre"}]}]}]
"#;

/// Maximum limit for payments list get api
pub const PAYMENTS_LIST_MAX_LIMIT_V1: u32 = 100;
/// Maximum limit for payments list post api with filters
pub const PAYMENTS_LIST_MAX_LIMIT_V2: u32 = 50;
/// Default limit for payments list API
pub fn default_payments_list_limit() -> u32 {
    10
}

/// Average delay (in seconds) between account onboarding's API response and the changes to actually reflect at Stripe's end
pub const STRIPE_ACCOUNT_ONBOARDING_DELAY_IN_SECONDS: i64 = 15;

/// Maximum limit for payment link list get api
pub const PAYMENTS_LINK_LIST_LIMIT: u32 = 100;

/// Maximum limit for payouts list get api
pub const PAYOUTS_LIST_MAX_LIMIT_GET: u32 = 100;
/// Maximum limit for payouts list post api
pub const PAYOUTS_LIST_MAX_LIMIT_POST: u32 = 20;
/// Default limit for payouts list API
pub fn default_payouts_list_limit() -> u32 {
    10
}

/// surcharge percentage maximum precision length
pub const SURCHARGE_PERCENTAGE_PRECISION_LENGTH: u8 = 2;

/// Header Key for application overhead of a request
pub const X_HS_LATENCY: &str = "x-hs-latency";

/// Default Payment Link Background color
pub const DEFAULT_BACKGROUND_COLOR: &str = "#212E46";

/// Default product Img Link
pub const DEFAULT_PRODUCT_IMG: &str =
    "https://live.hyperswitch.io/payment-link-assets/cart_placeholder.png";

/// Default Merchant Logo Link
pub const DEFAULT_MERCHANT_LOGO: &str =
    "https://live.hyperswitch.io/payment-link-assets/Merchant_placeholder.png";

/// Redirect url for Prophetpay
pub const PROPHETPAY_REDIRECT_URL: &str = "https://ccm-thirdparty.cps.golf/hp/tokenize/";

/// Variable which store the card token for Prophetpay
pub const PROPHETPAY_TOKEN: &str = "cctoken";

/// Default SDK Layout
pub const DEFAULT_SDK_LAYOUT: &str = "tabs";

/// Payment intent default client secret expiry (in seconds)
pub const DEFAULT_SESSION_EXPIRY: i64 = 15 * 60;

/// Payment intent fulfillment time (in seconds)
pub const DEFAULT_INTENT_FULFILLMENT_TIME: i64 = 15 * 60;

/// Payment order fulfillment time (in seconds)
pub const DEFAULT_ORDER_FULFILLMENT_TIME: i64 = 15 * 60;

/// Default bool for Display sdk only
pub const DEFAULT_DISPLAY_SDK_ONLY: bool = false;

/// Default bool to enable saved payment method
pub const DEFAULT_ENABLE_SAVED_PAYMENT_METHOD: bool = false;

/// Default allowed domains for payment links
pub const DEFAULT_ALLOWED_DOMAINS: Option<HashSet<String>> = None;

/// Default merchant details for payment links
pub const DEFAULT_TRANSACTION_DETAILS: Option<String> = None;

/// Default ttl for Extended card info  in redis (in seconds)
pub const DEFAULT_TTL_FOR_EXTENDED_CARD_INFO: u16 = 15 * 60;

/// Max ttl for Extended card info in redis (in seconds)
pub const MAX_TTL_FOR_EXTENDED_CARD_INFO: u16 = 60 * 60 * 2;

/// Default tenant to be used when multitenancy is disabled
pub const DEFAULT_TENANT: &str = "public";

/// Default tenant to be used when multitenancy is disabled
pub const TENANT_HEADER: &str = "x-tenant-id";

/// Max Length for MerchantReferenceId
pub const MAX_ALLOWED_MERCHANT_REFERENCE_ID_LENGTH: u8 = 64;

/// Maximum length allowed for a global id
pub const MIN_GLOBAL_ID_LENGTH: u8 = 32;

/// Minimum length required for a global id
pub const MAX_GLOBAL_ID_LENGTH: u8 = 64;

/// Minimum allowed length for MerchantReferenceId
pub const MIN_REQUIRED_MERCHANT_REFERENCE_ID_LENGTH: u8 = 1;

/// Length of a cell identifier in a distributed system
pub const CELL_IDENTIFIER_LENGTH: u8 = 5;

/// General purpose base64 engine
pub const BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;
/// Regex for matching a domain
/// Eg -
/// http://www.example.com
/// https://www.example.com
/// www.example.com
/// example.io
pub const STRICT_DOMAIN_REGEX: &str = r"^(https?://)?(([A-Za-z0-9][-A-Za-z0-9]\.)*[A-Za-z0-9][-A-Za-z0-9]*|(\d{1,3}\.){3}\d{1,3})+(:[0-9]{2,4})?$";

/// Regex for matching a wildcard domain
/// Eg -
/// *.example.com
/// *.subdomain.domain.com
/// *://example.com
/// *example.com
pub const WILDCARD_DOMAIN_REGEX: &str = r"^((\*|https?)?://)?((\*\.|[A-Za-z0-9][-A-Za-z0-9]*\.)*[A-Za-z0-9][-A-Za-z0-9]*|((\d{1,3}|\*)\.){3}(\d{1,3}|\*)|\*)(:\*|:[0-9]{2,4})?(/\*)?$";

/// Maximum allowed length for MerchantName
pub const MAX_ALLOWED_MERCHANT_NAME_LENGTH: usize = 64;

/// Default locale
pub const DEFAULT_LOCALE: &str = "en";

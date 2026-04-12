---
name: recipe-pricecheck
description: "Research active and sold Reverb listings to establish a used-market price range for purchasing intake."
metadata:
  version: 0.1.0-alpha.2
  openclaw:
    category: "recipe"
    domain: "purchasing"
    requires:
      bins:
        - revcli
      skills:
        - reverb-listings
---

# Price Check: Market Research for Purchasing Intake

> **PREREQUISITE:** Load the following skills to execute this recipe: `reverb-listings`

Research active listings and recent sold prices on Reverb.com to establish a used-market price range for gear submitted for possible purchase. Returns raw data with source links — the buyer makes the final offer judgment.

## Steps

1. Fetch active listings: `revcli listings list --query "MAKE MODEL" --per-page 50 --page-limit 4 --format json`
2. Fetch sold listings: `revcli listings list --query "MAKE MODEL" --params '{"show_only_sold":true}' --per-page 50 --page-limit 4 --format json`
3. For each result, extract:
   - `title` — listing title
   - `condition.display_name` — condition label (e.g. `"Excellent"`, `"Brand New"`)
   - `price.display` — human-readable sold/asking price (e.g. `"$7,899"`)
   - `original_price.display` — original listing price before any discounts (present on sold listings; compare against `price.display` to gauge discount depth)
   - `state.description` — `"Live"` for active, `"Sold"` for sold
   - `description` — HTML; strip tags and scan for condition caveats (replaced parts, repairs, modifications, damage)
   - `_links.web.href` — direct link to listing (cite this as source)
4. Present two separate price spreads (low → high), grouped by condition:
   - **Active listings** (`state.slug: "live"`) — current asking prices
   - **Sold listings** (`state.slug: "sold"`) — what buyers actually paid (primary baseline)
5. Annotate each cited listing with any condition qualifiers found in the description (e.g. "replaced tuners", "refret", "crack repair", "only taken out for photos")

## Parsing Notes

- **Exclude from used-market baseline**: listings where `condition.display_name` is `Brand New` or description indicates unplayed / new-old-stock — these do not reflect second-hand value
- **Flag with caveat, do not exclude**: listings noting replaced parts, repairs, or modifications — note the specific caveat alongside the price so the buyer can weigh it
- **Sold > active for baseline**: active listings are asking prices; sold listings are confirmed market transactions — weight sold data more heavily
- **Discount depth**: for sold listings with `original_price.display`, note the gap between original and sold price — a large spread signals that active listings at ask price are likely negotiable or overpriced
- **Always cite sources**: include `_links.web.href` for every listing used to establish the range
- **Description is HTML**: strip `<p>`, `<ul>`, `<li>`, `<br>` etc. before reading condition notes

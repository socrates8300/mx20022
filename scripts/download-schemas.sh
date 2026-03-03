#!/usr/bin/env bash
# Download ISO 20022 XSD schema files from the official ISO 20022 website.
#
# URL pattern:
#   https://www.iso20022.org/sites/default/files/documents/messages/{area}/schemas/{msg_id}.xsd
#
# Usage: ./scripts/download-schemas.sh [--force]
#   --force: overwrite existing files

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SCHEMAS_DIR="$PROJECT_ROOT/schemas"

BASE_URL="https://www.iso20022.org/sites/default/files/documents/messages"

FORCE=false
if [[ "${1:-}" == "--force" ]]; then
    FORCE=true
fi

# Schema definitions: area/msg_id
# We keep multiple versions per message for scheme compatibility:
#   - v08: FedNow baseline
#   - v10: CBPR+ / SEPA baseline
#   - latest: current ISO standard
SCHEMAS=(
    # Business Application Header
    "head/head.001.001.02"
    "head/head.001.001.04"

    # FI to FI Customer Credit Transfer (pacs.008)
    "pacs/pacs.008.001.08"
    "pacs/pacs.008.001.10"
    "pacs/pacs.008.001.13"

    # Payment Status Report (pacs.002)
    "pacs/pacs.002.001.10"
    "pacs/pacs.002.001.12"
    "pacs/pacs.002.001.14"

    # Return of Funds (pacs.004)
    "pacs/pacs.004.001.11"

    # FI to FI Financial Institution Credit Transfer (pacs.009)
    "pacs/pacs.009.001.10"

    # Status Report (pacs.028)
    "pacs/pacs.028.001.05"

    # Customer Credit Transfer Initiation (pain.001)
    "pain/pain.001.001.11"

    # Customer Payment Status Report (pain.002)
    "pain/pain.002.001.13"

    # Creditor Payment Activation Request (pain.013)
    "pain/pain.013.001.09"

    # Bank to Customer Statement (camt.053)
    "camt/camt.053.001.11"

    # Bank to Customer Debit Credit Notification (camt.054)
    "camt/camt.054.001.11"

    # FI to FI Payment Cancellation Request (camt.056)
    "camt/camt.056.001.11"

    # Resolution of Investigation (camt.029)
    "camt/camt.029.001.12"
)

download_count=0
skip_count=0
fail_count=0

for schema in "${SCHEMAS[@]}"; do
    area="${schema%%/*}"
    msg_id="${schema#*/}"
    filename="${msg_id}.xsd"
    target_dir="$SCHEMAS_DIR/$area"
    target_file="$target_dir/$filename"

    if [[ -f "$target_file" && "$FORCE" != "true" ]]; then
        echo "SKIP  $filename (exists, use --force to overwrite)"
        skip_count=$((skip_count + 1))
        continue
    fi

    mkdir -p "$target_dir"
    url="$BASE_URL/$area/schemas/$filename"

    echo -n "GET   $filename ... "
    http_code=$(curl -sSL -w "%{http_code}" -o "$target_file" "$url" 2>/dev/null)

    if [[ "$http_code" == "200" ]]; then
        # Validate it's actually XML
        if head -1 "$target_file" | grep -q '<?xml'; then
            size=$(wc -c < "$target_file" | tr -d ' ')
            echo "OK (${size} bytes)"
            download_count=$((download_count + 1))
        else
            echo "FAIL (not XML, got HTTP $http_code with non-XML content)"
            rm -f "$target_file"
            fail_count=$((fail_count + 1))
        fi
    else
        echo "FAIL (HTTP $http_code)"
        rm -f "$target_file"
        fail_count=$((fail_count + 1))
    fi
done

echo ""
echo "Done: $download_count downloaded, $skip_count skipped, $fail_count failed"

if [[ $fail_count -gt 0 ]]; then
    exit 1
fi

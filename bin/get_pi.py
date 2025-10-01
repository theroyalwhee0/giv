#!/usr/bin/env python3
"""
Download and verify the first 10,000 decimal places of Pi from multiple sources.
"""

import hashlib
import os
import re
import sys
from pathlib import Path
from urllib.request import Request, urlopen
from urllib.error import URLError

# Target number of decimal places
TARGET_PLACES = 10_000

# Cache directory
CACHE_DIR = Path(__file__).parent.parent / '.cache'

# User-Agent header to avoid 403 errors from some servers
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36'

# Pi sources - using high-precision sources (1M digits) to avoid rounding issues
SOURCES = [
    {
        'name': 'piday.org',
        'url': 'https://www.piday.org/million/',
        'parser': 'parse_html_continuous',
    },
    {
        'name': 'damienelliott.com',
        'url': 'https://www.damienelliott.com/1-million-digits-of-pi-%cf%80-ready-to-copy-and-paste/',
        'parser': 'parse_html_continuous',
    },
]


def ensure_cache_dir():
    """Ensure the cache directory exists."""
    CACHE_DIR.mkdir(parents=True, exist_ok=True)


def get_cache_path(url):
    """Get the cache file path for a given URL."""
    url_hash = hashlib.md5(url.encode()).hexdigest()
    return CACHE_DIR / f'pi_{url_hash}.html'


def download_url(url):
    """
    Download content from URL, using cache if available.

    Returns the content as a string.
    """
    cache_path = get_cache_path(url)

    # Check cache first
    if cache_path.exists():
        print(f'  Using cached file: {cache_path.name}')
        with open(cache_path, 'r', encoding='utf-8') as f:
            return f.read()

    # Download if not cached
    print(f'  Downloading from {url}')
    try:
        # Use a browser User-Agent to avoid 403 errors
        headers = {'User-Agent': USER_AGENT}
        request = Request(url, headers=headers)

        with urlopen(request, timeout=30) as response:
            content = response.read().decode('utf-8')

        # Cache the result
        with open(cache_path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f'  Cached to: {cache_path.name}')

        return content
    except URLError as e:
        print(f'  ERROR: Failed to download: {e}', file=sys.stderr)
        return None


def extract_digits(text):
    """
    Extract only digits from text, removing all other characters.
    """
    return ''.join(c for c in text if c.isdigit())


def parse_html_continuous(content):
    """
    Parse HTML pages with continuous Pi digits: "3.14159265358979..."

    Works for piday.org, damienelliott.com, and similar formats.

    Returns the decimal places (without the leading "3.").
    """
    # Find the first occurrence of "3.1415926535..." pattern with known start
    # This avoids false matches in HTML tags, metadata, etc.
    match = re.search(r'3\.1415926535(\d{' + str(TARGET_PLACES - 10) + r',})', content)
    if match:
        # We matched starting from the 11th decimal, prepend the first 10
        decimals = '1415926535' + match.group(1)
        return decimals[:TARGET_PLACES]

    # Try without the initial digits requirement
    match = re.search(r'3\.(\d{' + str(TARGET_PLACES) + r',})', content)
    if match:
        decimals = match.group(1)
        return decimals[:TARGET_PLACES]

    return None


def parse_txt(content):
    """
    Parse plain text files with Pi digits.

    May have spaces, line breaks, or other formatting.
    Looks for "3.14159..." pattern and extracts digits only.

    Returns the decimal places (without the leading "3.").
    """
    # Find the first occurrence starting with known Pi digits
    # Allow for spaces and newlines in the content
    match = re.search(r'3[.\s]*1415926535', content)
    if not match:
        return None

    # Start from this position and extract digits line by line
    start_pos = match.start()
    remaining = content[start_pos:]

    # Split into lines and process each line
    lines = remaining.split('\n')
    all_digits = []

    for line in lines:
        # Skip completely empty lines, marker lines, or dashed separator lines
        if not line.strip() or line.strip().startswith('*') or line.strip().startswith('-'):
            continue

        # For lines with annotations (like "<---- 50-th digit"),
        # extract only the digit part before the annotation
        if '<' in line:
            # Take everything before the '<' character
            line = line[:line.index('<')]

        # For lines with block markers (like "*1" at the end),
        # extract only the digit part before the marker
        if '*' in line:
            # Take everything before the '*' character
            line = line[:line.index('*')]

        # Extract digits from this line
        line_digits = extract_digits(line)
        if line_digits:
            all_digits.append(line_digits)

        # Stop once we have enough
        if len(''.join(all_digits)) >= TARGET_PLACES + 1:  # +1 for the leading 3
            break

    # Combine all digits
    result = ''.join(all_digits)

    # Remove leading 3
    if result.startswith('3'):
        result = result[1:]

    if len(result) < TARGET_PLACES:
        return None

    return result[:TARGET_PLACES]


# Map parser names to functions
PARSERS = {
    'parse_html_continuous': parse_html_continuous,
    'parse_txt': parse_txt,
}


def fetch_and_parse_source(source):
    """
    Fetch and parse a single source.

    Returns (source_name, decimals) or (source_name, None) on error.
    """
    name = source['name']
    url = source['url']
    parser_name = source['parser']

    print(f'\nFetching from {name}...')

    content = download_url(url)
    if content is None:
        return (name, None)

    parser = PARSERS[parser_name]
    decimals = parser(content)

    if decimals is None:
        print(f'  WARNING: Failed to parse Pi digits from content', file=sys.stderr)
        return (name, None)

    if len(decimals) < TARGET_PLACES:
        print(f'  WARNING: Only found {len(decimals)} decimal places', file=sys.stderr)
        return (name, None)

    print(f'  Successfully extracted {len(decimals)} decimal places')
    return (name, decimals)


def compare_results(results):
    """
    Compare results from all sources and verify they match.

    Returns the verified Pi decimals or None if there's a mismatch.
    """
    # Filter out failed sources
    valid_results = [(name, decimals) for name, decimals in results if decimals is not None]

    if not valid_results:
        print('\nERROR: No sources returned valid results', file=sys.stderr)
        return None

    if len(valid_results) == 1:
        print('\nWARNING: Only one source available, cannot cross-verify', file=sys.stderr)
        return valid_results[0][1]

    # Compare all sources
    print(f'\nComparing {len(valid_results)} sources...')

    # First, verify all are digits only
    print('\nValidating extracted decimals...')
    for name, decimals in valid_results:
        if not decimals.isdigit():
            print(f'  ✗ {name}: Contains non-digit characters!', file=sys.stderr)
            # Show first non-digit
            for i, c in enumerate(decimals):
                if not c.isdigit():
                    print(f'    First non-digit at position {i}: {repr(c)}', file=sys.stderr)
                    break
        else:
            print(f'  ✓ {name}: All digits valid')

    # Check lengths
    print('\nLengths:')
    for name, decimals in valid_results:
        print(f'  {name}: {len(decimals)} decimal places')
        print(f'    First 20: {decimals[:20]}')
        print(f'    Last 20: {decimals[-20:]}')

    reference_name, reference_decimals = valid_results[0]

    all_match = True
    for name, decimals in valid_results[1:]:
        if decimals == reference_decimals:
            print(f'  ✓ {name} matches {reference_name}')
        else:
            print(f'  ✗ {name} DIFFERS from {reference_name}', file=sys.stderr)
            print(f'    Length: {len(decimals)} vs {len(reference_decimals)}', file=sys.stderr)
            # Find first difference
            for i, (d1, d2) in enumerate(zip(reference_decimals, decimals)):
                if d1 != d2:
                    print(f'    First difference at position {i}: {d1} vs {d2}', file=sys.stderr)
                    break
            all_match = False

    if not all_match:
        print('\nERROR: Sources disagree on Pi digits!', file=sys.stderr)
        return None

    print('\n✓ All sources agree!')
    return reference_decimals


def parse_decimals_rs(file_path):
    """
    Parse the PI_DECIMALS array from src/c_pi/decimals.rs.

    Returns the decimal places as a string, or None if parsing fails.
    """
    try:
        with open(file_path, 'r') as f:
            content = f.read()

        # Find the PI_DECIMALS array definition
        # Look for: pub const PI_DECIMALS: [u8; 10_000] = [
        match = re.search(r'pub const PI_DECIMALS:\s*\[u8;\s*\d+_?\d*\]\s*=\s*\[([\d,\s]+)\];', content, re.DOTALL)
        if not match:
            return None

        # Extract the array contents
        array_str = match.group(1)

        # Extract all digits, ignoring commas and whitespace
        digits = extract_digits(array_str)

        return digits
    except FileNotFoundError:
        return None
    except Exception as e:
        print(f'  ERROR parsing decimals.rs: {e}', file=sys.stderr)
        return None


def main():
    """Main entry point."""
    print(f'Pi Digit Downloader and Verifier')
    print(f'Target: First {TARGET_PLACES:,} decimal places')

    ensure_cache_dir()

    # Fetch and parse all sources
    results = []
    for source in SOURCES:
        result = fetch_and_parse_source(source)
        results.append(result)

    # Compare and verify
    verified_decimals = compare_results(results)

    if not verified_decimals:
        print('\nFailed to verify Pi digits', file=sys.stderr)
        return 1

    print(f'\nVerified Pi decimals (first 100):')
    print(f'3.{verified_decimals[:100]}...')
    print(f'\nTotal decimal places: {len(verified_decimals):,}')

    # Check if decimals.rs exists and verify it
    decimals_rs_path = Path(__file__).parent.parent / 'src' / 'c_pi' / 'decimals.rs'
    if decimals_rs_path.exists():
        print(f'\nVerifying existing decimals.rs...')
        existing_decimals = parse_decimals_rs(decimals_rs_path)

        if existing_decimals is None:
            print(f'  WARNING: Failed to parse decimals.rs', file=sys.stderr)
        else:
            print(f'  Found {len(existing_decimals):,} decimal places in decimals.rs')

            # Compare with verified decimals
            if existing_decimals == verified_decimals:
                print(f'  ✓ decimals.rs matches verified Pi digits!')
            else:
                print(f'  ✗ decimals.rs DIFFERS from verified Pi digits!', file=sys.stderr)

                # Find first difference
                min_len = min(len(existing_decimals), len(verified_decimals))
                for i in range(min_len):
                    if existing_decimals[i] != verified_decimals[i]:
                        print(f'    First difference at position {i}: {existing_decimals[i]} vs {verified_decimals[i]}', file=sys.stderr)
                        print(f'      decimals.rs: ...{existing_decimals[max(0,i-10):i+10]}...', file=sys.stderr)
                        print(f'      verified:    ...{verified_decimals[max(0,i-10):i+10]}...', file=sys.stderr)
                        break

                if len(existing_decimals) != len(verified_decimals):
                    print(f'    Length difference: {len(existing_decimals)} vs {len(verified_decimals)}', file=sys.stderr)
    else:
        print(f'\nNote: decimals.rs not found at {decimals_rs_path}')

    # Output to stdout for piping
    print(f'\nFull output:')
    print(verified_decimals)
    return 0


if __name__ == '__main__':
    sys.exit(main())

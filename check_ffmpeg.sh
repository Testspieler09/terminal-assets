#!/usr/bin/env bash
set -euo pipefail

PASS="✓"
FAIL="✗"
WARN="!"
errors=0

check() {
    local label="$1"
    local cmd="$2"
    if eval "$cmd" &>/dev/null; then
        echo "  $PASS $label"
    else
        echo "  $FAIL $label"
        errors=$((errors + 1))
    fi
}

echo ""
echo "ffmpeg"
if ! command -v ffmpeg &>/dev/null; then
    echo "  $FAIL ffmpeg not found in PATH"
    echo ""
    echo "Install with:"
    case "$(uname -s)" in
        Darwin) echo "  brew install ffmpeg" ;;
        Linux)  echo "  sudo apt install ffmpeg   # Debian/Ubuntu"
                echo "  sudo dnf install ffmpeg   # Fedora" ;;
        *)      echo "  See https://ffmpeg.org/download.html" ;;
    esac
    exit 1
fi
check "ffmpeg found at $(command -v ffmpeg)" "command -v ffmpeg"
echo "  $PASS version: $(ffmpeg -version 2>&1 | head -1)"

echo ""
echo "Codecs"
CODECS="$(ffmpeg -codecs 2>/dev/null)"
FILTERS="$(ffmpeg -filters 2>/dev/null)"

check "H.264 (libx264)"    "grep -q 'libx264'    <<< \"\$CODECS\""
check "VP9   (libvpx-vp9)" "grep -q 'libvpx-vp9' <<< \"\$CODECS\""
check "GIF encoding"       "grep -qE '.E.*\bgif\b' <<< \"\$CODECS\""
check "palettegen filter"  "grep -q 'palettegen'  <<< \"\$FILTERS\""
check "paletteuse filter"  "grep -q 'paletteuse'  <<< \"\$FILTERS\""

echo ""
echo "Shared libraries"
FFMPEG_BIN="$(command -v ffmpeg)"
case "$(uname -s)" in
    Darwin)
        missing_libs=()
        while IFS= read -r line; do
            lib_path=$(echo "$line" | awk '{print $1}')
            if [[ "$lib_path" == /opt/homebrew/* || "$lib_path" == /usr/local/* ]]; then
                if [[ ! -f "$lib_path" ]]; then
                    missing_libs+=("$lib_path")
                fi
            fi
        done < <(otool -L "$FFMPEG_BIN" 2>/dev/null | tail -n +2)

        if [[ ${#missing_libs[@]} -eq 0 ]]; then
            echo "  $PASS all dylibs resolved"
        else
            echo "  $FAIL missing dylibs:"
            for lib in "${missing_libs[@]}"; do
                echo "       $lib"
            done
            echo ""
            echo "  Fix with: brew reinstall ffmpeg"
            errors=$((errors + 1))
        fi
        ;;
    Linux)
        if ldd "$FFMPEG_BIN" 2>/dev/null | grep -q "not found"; then
            echo "  $FAIL missing shared libraries:"
            ldd "$FFMPEG_BIN" 2>/dev/null | grep "not found" | sed 's/^/       /'
            errors=$((errors + 1))
        else
            echo "  $PASS all shared libraries resolved"
        fi
        ;;
    *)
        echo "  $WARN shared library check not supported on this OS"
        ;;
esac

echo ""
if [[ $errors -eq 0 ]]; then
    echo "All checks passed."
else
    echo "$errors check(s) failed."
    exit 1
fi

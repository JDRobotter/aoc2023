#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    uint32_t a;
    uint32_t b;
} range_t;

int parse_range(range_t *range, const char *s, size_t len) {

    // find occurence of '-'
    char *dash = memchr(s, '-', len);
    if (dash == NULL) {
        return -1;
    }

    char *endptr;
    // parse from start of string to dash as an integer
    endptr = dash;
    range->a = strtol(s, &endptr, 10);
    assert(endptr == dash);

    // parse from dash to end of string as an integer
    endptr = (char *)s + len;
    range->b = strtol(dash + 1, &endptr, 10);
    assert(*endptr == '\0' || endptr == (char *)s + len);

    return 0;
}

size_t parse_input(range_t ranges[], size_t len, const char *input) {

    const char *rp = input;
    size_t input_len = strlen(input);
    size_t i;
    const char *end = input + input_len;
    for (i = 0; i < len; i++) {
        // find next occurence of ','
        const char *next = strchr(rp, ',');

        size_t range_len = next - rp;
        if (next == NULL) {
            // no ',' found, we reached the end of input
            range_len = end - rp;
        }
        if (parse_range(&ranges[i], rp, range_len) < 0) {
            return -1;
        };

        // end of input found
        if (next == NULL) {
            break;
        }

        // move read pointer to end of parsed range
        rp = next + 1;
    }

    return i + 1;
}

bool is_id_valid(const char *s) {

    size_t n = strlen(s);

    // if ID length is odd, ID is valid
    if (n % 2 != 0) {
        return true;
    }

    size_t mid = n / 2;

    if (memcmp(s, s + mid, mid) == 0) {
        // left and right part are identical
        return false;
    } else {
        return true;
    }
}

uint32_t invalids_in_range(range_t *r) {

    size_t sum = 0;
    // iterate through all IDs in range
    for (uint32_t i = r->a; i <= r->b; i++) {
        // declare a buffer
        char buffer[256] = {0};
        // print string in buffer
        snprintf(buffer, sizeof(buffer), "%u", i);
        // check if ID is valid
        if (!is_id_valid(buffer)) {
            fprintf(stderr, "INV %u\n", i);
            sum += i;
        }
    }

    return sum;
}

int main(int argc, char **argv) {
    const char *input =
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,"
        "1698522-1698528,446443-446449,38593856-38593862,565653-565659,"
        "824824821-824824827,2121212118-2121212124";

    range_t ranges[1024] = {(range_t){-1, -1}};

    size_t n = parse_input(ranges, 1024, input);
    assert(n >= 0);

    size_t sum = 0;
    for (size_t i = 0; i < n; i++) {
        range_t *r = &ranges[i];
        // find how many IDs in range are invalids
        sum += invalids_in_range(r);
    }
    fprintf(stderr, "invalids = %ld\n", sum);

    return 0;
}

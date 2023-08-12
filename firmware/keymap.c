uint8_t r;
uint8_t g;
uint8_t b;
bool received_data = false;

void raw_hid_receive(uint8_t *data, uint8_t length) {
    r = data[0];
    g = data[1];
    b = data[2];

    if (!received_data) {
        received_data = true;
    }
}

bool rgb_matrix_indicators_advanced_user(uint8_t led_min, uint8_t led_max) {
    if (!received_data) {
        return false;
    }

    for (uint8_t i = led_min; i < led_max; i++) {
        rgb_matrix_set_color(i, r, g, b);
    }

    return false;
}

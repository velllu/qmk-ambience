typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} RGB_ambience;

bool received_data = false;

RGB_ambience current_color;
RGB_ambience step;

RGB_ambience step_color(RGB_ambience color_from, RGB_ambience color_to) {
    RGB_ambience new_color = color_from;

    // Yes, I am not a C developer, how could you tell?
    // This actually works surprisingly well, i found this to be far smoother then a lineaar intepolation
    // function
    if (color_from.r < color_to.r) { new_color.r += 1; } else if (color_from.r > color_to.r) { new_color.r -= 1; }
    if (color_from.g < color_to.g) { new_color.g += 1; } else if (color_from.g > color_to.g) { new_color.g -= 1; }
    if (color_from.b < color_to.b) { new_color.b += 1; } else if (color_from.b > color_to.b) { new_color.b -= 1; }

    return new_color;
}

void raw_hid_receive(uint8_t *data, uint8_t length) {
    current_color.r = data[0];
    current_color.g = data[1];
    current_color.b = data[2];

    if (!received_data) {
        received_data = true;
    }
}

bool rgb_matrix_indicators_advanced_user(uint8_t led_min, uint8_t led_max) {
    if (!received_data) {
        return false;
    }

    step = step_color(step, current_color);

    for (uint8_t i = led_min; i < led_max; i++) {
        rgb_matrix_set_color(i, step.r, step.g, step.b);
    }

    return false;
}


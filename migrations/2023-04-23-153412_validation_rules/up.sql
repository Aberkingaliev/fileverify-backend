CREATE TABLE extension_list (
    id SERIAL PRIMARY KEY,
    extension VARCHAR(255) NOT NULL
);

CREATE TABLE validation_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    min_size BIGINT NOT NULL,
    max_size BIGINT NOT NULL,
    allowed_extension_id INTEGER NOT NULL,
    FOREIGN KEY (allowed_extension_id) REFERENCES extension_list (id)
);

CREATE TABLE advance_options (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    validation_rule_id UUID NOT NULL,
    extension_id INTEGER NOT NULL,
    is_email_validate BOOLEAN NOT NULL,
    FOREIGN KEY (validation_rule_id) REFERENCES validation_rules (id) ON DELETE CASCADE,
    FOREIGN KEY (extension_id) REFERENCES extension_list (id)
);


CREATE TABLE keywords (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    advance_option_id UUID NOT NULL,
    keyword VARCHAR(255) NOT NULL,
    FOREIGN KEY (advance_option_id) REFERENCES advance_options (id) ON DELETE CASCADE
);

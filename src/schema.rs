table! {
    address (address_id) {
        address_id -> Int4,
        address_line_1 -> Nullable<Text>,
        address_line_2 -> Nullable<Text>,
        county -> Nullable<Text>,
        city -> Nullable<Text>,
        postal_code -> Nullable<Text>,
        business_name -> Nullable<Text>,
        latitude -> Nullable<Text>,
        longitude -> Nullable<Text>,
    }
}

table! {
    mna (mna_id) {
        mna_id -> Int4,
        area_code -> Text,
        digits -> Int4,
        description -> Text,
        towns -> Text,
        area -> Text,
    }
}

table! {
    number_blocks (numberblock_id) {
        numberblock_id -> Int4,
    }
}

table! {
    number_status (number_status_id) {
        number_status_id -> Int4,
        number_status_name -> Text,
    }
}

table! {
    numbers (number_id) {
        number_id -> Int4,
        number -> Text,
        number_type -> Int4,
        number_status -> Int4,
        prefix_id -> Nullable<Int4>,
        status_change -> Nullable<Timestamptz>,
        block_holder -> Nullable<Text>,
        comments -> Nullable<Text>,
        mna_id -> Int4,
        subscriber_id -> Nullable<Int4>,
        numberblock_id -> Nullable<Int4>,
        wlr -> Bool,
        is_main_number -> Bool,
        ecas -> Bool,
        ndd -> Bool,
    }
}

table! {
    portings (porting_id) {
        porting_id -> Int4,
        number_id -> Nullable<Int4>,
        numberblock_id -> Nullable<Int4>,
        porting_from -> Text,
        porting_to -> Text,
        status -> Int4,
        porting_start -> Timestamptz,
        porting_event_date -> Nullable<Timestamptz>,
        porting_completion -> Nullable<Timestamptz>,
        comments -> Nullable<Text>,
    }
}

table! {
    routing_prefix (prefix_id) {
        prefix_id -> Int4,
        prefix -> Text,
    }
}

table! {
    rsp (rsp_id) {
        rsp_id -> Int4,
        address_id -> Nullable<Int4>,
        name -> Text,
        account -> Nullable<Text>,
    }
}

table! {
    subscribers (subscriber_id) {
        subscriber_id -> Int4,
        address_id -> Nullable<Int4>,
        rsp_id -> Nullable<Int4>,
        name -> Text,
        accountid -> Text,
        is_business -> Bool,
        premise_id -> Nullable<Text>,
        eircode_id -> Nullable<Text>,
        ard_id -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

table! {
    tokens (token_id) {
        token_id -> Text,
        username -> Text,
        expiry -> Timestamptz,
    }
}

table! {
    users (username) {
        username -> Text,
        password -> Text,
        email -> Text,
    }
}

joinable!(numbers -> mna (mna_id));
joinable!(numbers -> number_blocks (numberblock_id));
joinable!(numbers -> routing_prefix (prefix_id));
joinable!(numbers -> subscribers (subscriber_id));
joinable!(portings -> number_blocks (numberblock_id));
joinable!(portings -> numbers (number_id));
joinable!(rsp -> address (address_id));
joinable!(subscribers -> address (address_id));
joinable!(subscribers -> rsp (rsp_id));

allow_tables_to_appear_in_same_query!(
    address,
    mna,
    number_blocks,
    numbers,
    portings,
    routing_prefix,
    rsp,
    subscribers,
    tokens,
    users,
);

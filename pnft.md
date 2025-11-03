Example of a frozen (staked) pNFT:
```
ATA is WgXEJjLHQAj5B7cNrjnVKg11CkgM5kWUY13ukKmNmBC:
Account {
    mint: 8ctufeeJWrBaqDikM2cUjxN9muvRK2pgQs1KQwEFgYDU,
    owner: 6hb8Vr2dKSQr1mmRkNz4WxSFVvjs7t2iju928c4Gupep,
    amount: 1,
    delegate: Some(
        3D4kDH3Mut6ZsWkWGrzXR47Qy6YpmX1kHmzA4hXNzopG,
    ),
    state: Frozen,
    is_native: None,
    delegated_amount: 1,
    close_authority: None,
}
TRA is HRSCk9s2StP5LFdxvMBLuvUaNe4vsZ1zvQrjadTe46zb:
TokenRecord {
    key: TokenRecord,
    bump: 254,
    state: Locked,
    rule_set_revision: Some(
        1,
    ),
    delegate: Some(
        3D4kDH3Mut6ZsWkWGrzXR47Qy6YpmX1kHmzA4hXNzopG,
    ),
    delegate_role: Some(
        Staking,
    ),
    locked_transfer: None,
}
Metadata is HbvUJ2qRRu9q6hRzuqZRgKbqF24JFaBTnujhgKWciuVk:
Metadata {
    key: MetadataV1,
    update_authority: PSG1TGSniaZTFdzcxz8G5JaQ4oa8k2cSa9YZiqKTGu6,
    mint: 8ctufeeJWrBaqDikM2cUjxN9muvRK2pgQs1KQwEFgYDU,
    name: "Player1 #1424\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    symbol: "PSP1\0\0\0\0\0\0",
    uri: "https://bafybeih7oz5pds33io34ud5ut4fd5a2jj74zoduiec2fw4oz24yqfo4aey.ipfs.w3s.link/1424.json\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    seller_fee_basis_points: 500,
    creators: Some(
        [
            Creator {
                address: GzHDxTG83o99CpH34nD1rwPcFXfuy9NX8wxVjQDxDBXN,
                verified: true,
                share: 0,
            },
            Creator {
                address: 71TwiZBxrYnijtW8Q7ML9gMrDjfXDwXD3pvJ2xKks9Pg,
                verified: false,
                share: 100,
            },
        ],
    ),
    primary_sale_happened: true,
    is_mutable: true,
    edition_nonce: Some(
        255,
    ),
    token_standard: Some(
        ProgrammableNonFungible,
    ),
    collection: Some(
        Collection {
            verified: true,
            key: PSP1JmwMdd3usfyYMXTXfnRNs4e1M1UeVo7hQRaFmLu,
        },
    ),
    uses: None,
    collection_details: None,
    programmable_config: Some(
        V1 {
            rule_set: Some(
                2mTEinDCzhPF44BJpxH9hWpmisy5XpXzw8kKzB6ZAWwb,
            ),
        },
    ),
}
```

Example of a pNFT that is not frozen and is listed in magic eden:

```
ATA is F7UmoSCSzmuJjq719RNBof4RSyUEhdwnsM7dvPdohSaD:
Account {
    mint: DsSMGN8A2BAYgYYJpkke5wWnfszpPN9rZH2ciPabZszp,
    owner: Dt5bTx5jkW8TqFmJBKYfeKnMKTfoQypjViBNywQZU8PL,
    amount: 1,
    delegate: None,
    state: Frozen,
    is_native: None,
    delegated_amount: 0,
    close_authority: None,
}
TRA is A9obnw3mubyqWqfX89TyWPkjmVuTX5n1W3yUKtUX9YCX:
TokenRecord {
    key: TokenRecord,
    bump: 253,
    state: Unlocked,
    rule_set_revision: None,
    delegate: None,
    delegate_role: None,
    locked_transfer: None,
}
Metadata is CxkXyHx6z5xzWh2KMvfsyVVUJ81hjfWRajUBgP61qDoz:
Metadata {
    key: MetadataV1,
    update_authority: PSG1TGSniaZTFdzcxz8G5JaQ4oa8k2cSa9YZiqKTGu6,
    mint: DsSMGN8A2BAYgYYJpkke5wWnfszpPN9rZH2ciPabZszp,
    name: "Player1 #925\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    symbol: "PSP1\0\0\0\0\0\0",
    uri: "https://bafybeih7oz5pds33io34ud5ut4fd5a2jj74zoduiec2fw4oz24yqfo4aey.ipfs.w3s.link/925.json\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    seller_fee_basis_points: 500,
    creators: Some(
        [
            Creator {
                address: GzHDxTG83o99CpH34nD1rwPcFXfuy9NX8wxVjQDxDBXN,
                verified: true,
                share: 0,
            },
            Creator {
                address: 71TwiZBxrYnijtW8Q7ML9gMrDjfXDwXD3pvJ2xKks9Pg,
                verified: false,
                share: 100,
            },
        ],
    ),
    primary_sale_happened: true,
    is_mutable: true,
    edition_nonce: Some(
        255,
    ),
    token_standard: Some(
        ProgrammableNonFungible,
    ),
    collection: Some(
        Collection {
            verified: true,
            key: PSP1JmwMdd3usfyYMXTXfnRNs4e1M1UeVo7hQRaFmLu,
        },
    ),
    uses: None,
    collection_details: None,
    programmable_config: Some(
        V1 {
            rule_set: Some(
                2mTEinDCzhPF44BJpxH9hWpmisy5XpXzw8kKzB6ZAWwb,
            ),
        },
    ),
}
```

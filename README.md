This is a utility that leverages surfpool to overwrite the bytes in a core NFT to change its ownership.

# Help

`cargo run -- -h`

# Example

Set the owner of the nft `11111111111111111111111111111111` to be `22222222222222222222222222222222`

`cargo run rob-core-nft 11111111111111111111111111111111 22222222222222222222222222222222`



TODO:

- [ ] For now you can only deserialize, serialize and change the main data, and not any plugins. I'm working on another crate to allow this since the main mpl crate is really bad

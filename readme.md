### QR Code Board games generator

I just had a silly idea to make every board game I own a NFT.

So after I created the NFT I got the idea to print a QR code on the board game box and link it to the NFT.

I also wanted to practice my RUST skills so I created this little tool to generate QR codes.

## Usage

```bash
cargo build -r
./target/release/qrcode-boardgames-nft output_file.pdf {ADDRESS}
```
where `{ADDRESS}` is the address of the NFTs owner on MultiversX blockchain.

## Example (for my wallet)
```bash
./target/release/qrcode-boardgames-nft nfts.pdf erd158lk5s2m3cpjyg5fwgm0pwnt8ugnc29mj4nafkrvcrhfdfpgvp3swpmnrj
```


## View my board game collection
You can view my board game collection on [MultiversX xSpotlight](https://xspotlight.com/collections/BOARD-25bcd6)
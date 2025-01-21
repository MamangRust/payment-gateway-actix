### Example Payment Gateway SeaOrm + Actix Web



### Cara Kerjanya Payment Gateway ini yang menggunakan actix + seaorm
<img src="./payment_gateway.png" alt="payment" />



### Migrate

```sh
sea-orm-cli migrate init
```

### Generate Entity

```sh
sea-orm-cli generate entity -o src/entities
```

### Migrate run UP

```sh
sea-orm-cli migrate up
```

### Migrate run Down

```sh
sea-orm-cli migrate down
```

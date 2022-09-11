# CORS middleware

To allow sites in the CORS middleware go to the project's root and create a `.env` file, then add the `CORS_ORIGIN` key.

For example:

```
// .env
CORS_ORIGIN=127.0.0.1
```

You can add multiple sites using commas between each site, for example:

```
CORS_ORIGIN=127.0.0.1,https://doc.rust-lang.org
```

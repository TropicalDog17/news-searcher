## News Searcher

Information Retrieval

Techstack:
   + Crawler: Scrapy(Python)

   + Search engine: Tantivy(engine), Axum(API), both in Rust

   + UI: Next.js

   + Database: sqlx, Postgres

### HOW TO RUN

Required: Rust, Python, Node.js environment. Stable release is recommended.
The instruction is for Ubuntu, please do research for other platforms.

1. clone the repository
2. install psql, postgresql
   ```bash
   sudo apt install postgresql postgresql-contrib
   cargo add sqlx-cli
   ```
3. scaffold the tables

   ```bash
   cd search-engine
   sqlx db setup
   ```
5. import the db

   ```bash
   psql -h localhost -p 5432 -d articles -U postgres
   ```

   ```psql
   articles# \copy article (id,title,summary,content,created_time,url) from 'path/to/news.csv/file' header csv delimiter ',';
   ```

6. run the api

   ```bash
   cargo run
   ```

7. run the ui
   ```bash
   cd ui
   npm run dev
   ```

The search engine will be ready to be used at `localhost:3000`

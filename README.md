# photo_backup_server

A photo and video backup server written in Rust. It handles accounts, chunked uploads and
downloads, media metadata, thumbnails, and video streaming over a JSON API.

## Features

- **Auth**: registration and login with bcrypt-hashed passwords, JWT access tokens, refresh
  tokens, and login activity tracking
- **Uploads and downloads**: chunked transfers, so large files don't need a single request
- **Metadata**: EXIF data for photos and ffprobe data for videos
- **Thumbnails**: WebP thumbnails for photos and videos, generated on first request
- **Streaming**: stream videos instead of downloading them in full
- **Paginated listing** of each user's media

## Stack

Rust · [Axum](https://github.com/tokio-rs/axum) · Tokio · [SQLx](https://github.com/launchbadge/sqlx) ·
PostgreSQL · jsonwebtoken · ffmpeg

## API

| Method | Route | Description |
| --- | --- | --- |
| `POST` | `/auth/register` | Create an account |
| `POST` | `/auth/login` | Log in and receive tokens |
| `POST` | `/auth/refresh-tokens` | Exchange a refresh token for new tokens |
| `GET` | `/auth/verify` | Check an access token |
| `GET` | `/user/self` | Current user |
| `POST` | `/media` | Upload a chunk |
| `POST` | `/media/list` | List media with pagination |
| `GET` | `/media/{id}` | Media details and metadata |
| `POST` | `/media/{id}/download` | Download a chunk |
| `GET` | `/media/{id}/thumbnail` | Get or generate a thumbnail |
| `GET` | `/media/{id}/stream` | Stream a video |

All `/media` and `/user` routes need an access token.

## Running locally

Requirements: Rust, PostgreSQL, `ffmpeg`, and `sqlx-cli`.

```bash
cp env.example .env           # fill in the database, JWT, and port settings
cargo install sqlx-cli
cargo migration:r             # runs the migrations (alias for `sqlx migrate run`)
cargo run
```

For development with auto-reload, install `cargo-watch` and run `cargo start:dev`.
Uploaded files are stored under `./uploads`.

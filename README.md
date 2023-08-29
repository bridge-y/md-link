# MD-link

A Clouflare Workers service to fetch web page titles and return them as markdown links.

## Setup

1. Click the button below and follow the on-screen instructions to proceed with the setup.

   [![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/bridge-y/bluesky-worker)

2. On the Cloudflare Workers dashboard, register the following secrets as variables.

   - `REQUEST_PATH`: A randomly generated value.

## Usage

```bash
curl https://<your worker domain>/<REQUEST_PATH>\?url=<url you want to get markdown link>
```

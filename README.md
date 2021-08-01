# stocktools-rs
Tools for tracking and viewing stocks written entirely in Rust!

### Pre-requisites
All tools require an API key from Alpha-Vantage, you can grab one [here](https://www.alphavantage.co/support/#api-key).

Place they key in a file in the root directory named:

`./alpha_vantage_key`

This file is untracked by Git so don't worry about someone stealing it.

## Tools
### Stockgraph

This tool takes a security's ticker as the sole argument and displays a rough graph of its
movement through the day up to the current time.

Usage:
`stockgraph <ticker>`


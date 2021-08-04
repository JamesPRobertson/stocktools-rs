# stocktools-rs
Tools for tracking and viewing stocks written entirely in Rust!

### Prerequisites
All tools require an API key from Alpha-Vantage, you can grab one [here](https://www.alphavantage.co/support/#api-key).

Place the key in a file in the root directory named:

`./alpha_vantage_key`

This file is untracked by Git so don't worry about someone stealing it.

## Tools
### Stockgraph

This tool takes a security's ticker as the sole argument and displays a rough graph of its
movement through the day up to the current time.

For now, the graph shows 5 minute movements.

At this moment, the Cargo points to Stockgraph by default so usage is:

`cargo run <ticker>`

Usage in the future will be:

`stockgraph <ticker>`

## Upcoming Features
### Stockgraph
General improvments to the graph.
- Proper candles showing the highs and lows within the timeframe.
- Configurable height and width of the graph.
- Historical data - given a date in the past

# TODO: PLEASE UPDATE THIS
### Stockline
A single line that updates with the security's movement up to 5 times per minute!
- Multiline version that tracks multiple securities.

### Unified Stocktools
Combining all the available tools into a single binary. Using or selecting the different
tools will be done with arguments, for example:
- To display the Stockgraph on a given date defaulted to the most recent trading day:
   - `stocktools-rs graph $<ticker> --date <date>`
- To run Stockline for a given list of securities.
   - `stocktools-rs line $<list of tickers>`

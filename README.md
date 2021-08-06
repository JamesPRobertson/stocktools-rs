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

To use Stockgraph, use this command within stocktools-rs/

`cargo run graph <ticker>`

### Stockwatch

This tool is currently under development while a few more bugs are sorted out. This
tool will also be limited by Alpha-Vantage's API call limit of 5 per minute. Therefore,
if you are tracking more than 5 tickers, the updates will be quite slow.

But for now, this tool will display as many tickers as you can fit on your screen
and display a line of changing green or red squares depending on the security's movement.

Please note: to exit this tool, use <ctrl-c> to exit.

To invoke this tool, use the command

`cargo run watch [<tickers>]`

You can enter as many tickers as you have room to view them, seperated by a space.

## Upcoming Features
### Stockgraph
General improvments to the graph.
- Proper candles showing the highs and lows within the timeframe.
- Configurable height and width of the graph.
- Historical data - given a date in the past

### Stockwatch

- Actually having the updates pull in real data.
- Flashing the color of the movement on the ticker itself.
- Ability to add or remove tickers from the list at runtime
   - This also gives the possibility of exiting the program gracefully

### Unified Stocktools
Combining all the available tools into a single binary. Using or selecting the different
tools will be done with arguments, for example:
- To display the Stockgraph on a given date defaulted to the most recent trading day:
   - `stocktools-rs graph $<ticker> --date <date>`
- To run Stockline for a given list of securities.
   - `stocktools-rs line $<list of tickers>`

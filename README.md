# Fintools - ISIN Converter for Tradingview
## Features
### Conversion
This converter takes a list of ISIN's and converts them in to Trading-View-Style Watchlist. It caches known symbols two avoid network roundtrips. At the moment only the 3 main exchanes ARCA, NYSE and NASDAQ are supported, but others can be added on your own or on feature request (please add examples and exchange symbols).
### Comparison
As a second feature, two lists generated before can be compared to each other for differences. This returns a list with two sections of new and gone symbols.
## Installation
I can not provide an executable file, because the heuristic of Microsoft Defender will mark it as Virus/Malware. 
1. Please compile it by yourself by installing [Rust](https://www.rust-lang.org/tools/install) with RustUp.
2. Clone Repo
3. Run ``cargo clean -r``
4. Run ``build -r``
5. Copy the Executable from subfolder ``target/release/fintools.exe`` to your desired location.
6. Run it.


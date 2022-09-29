#!/usr/bin/python3

import csv
from sys import argv
from typing import List, Tuple

CMD_GET_COLUMN = "get-column"
CMD_FILTER = "filter"

USAGE = f"""Usage: pypanda.py <csv_file> <command> [cmdargs...]

Where command can be:

    - {CMD_GET_COLUMN} <column-name> [unique]: collect CSV columns by name. If unique is provided, only unique values are returned
    - {CMD_FILTER} <column-name> <value>...: collect CSV columns which `column-name` has value `value`
"""


class CsvData(object):
    def __init__(self, csv_file: str) -> None:
        self.trades: List[Trade] = []
        with open(csv_file, newline="") as csvfile:
            reader = csv.reader(csvfile, delimiter=",")
            for row in reader:
                try:
                    self.trades.append(Trade(row))
                except Exception:
                    continue


class Trade(object):
    def __init__(self, row: List[str]) -> None:
        self.transaction_id = row[0]
        self.timestamp = row[1]
        self.transaction_type = row[2]
        self.in_out = row[3]
        self.amount_fiat = row[4]
        self.fiat = row[5]
        self.amount_asset = row[6]
        self.asset = row[7]
        self.asset_market_price = row[8]
        self.asset_market_price_currency = row[9]
        self.asset_class = row[10]
        self.product_id = row[11]
        self.fee = row[12]
        self.fee_asset = row[13]
        self.spread = row[14]
        self.spread_currency = row[15]

    def __repr__(self) -> str:
        return f"{self.transaction_id},{self.timestamp},{self.transaction_type},{self.in_out},{self.amount_fiat},{self.fiat},{self.amount_asset},{self.asset},{self.asset_market_price},{self.asset_market_price_currency},{self.asset_class},{self.product_id},{self.fee},{self.fee_asset},{self.spread},{self.spread_currency}"


def get_column(csv_data: CsvData, column: str) -> List[str]:
    if column == "transaction_id":
        return list(map(lambda x: x.transaction_id, csv_data.trades))
    elif column == "timestamp":
        return list(map(lambda x: x.timestamp, csv_data.trades))
    elif column == "transaction_type":
        return list(map(lambda x: x.transaction_type, csv_data.trades))
    elif column == "in_out":
        return list(map(lambda x: x.in_out, csv_data.trades))
    elif column == "amount_fiat":
        return list(map(lambda x: x.amount_fiat, csv_data.trades))
    elif column == "fiat":
        return list(map(lambda x: x.fiat, csv_data.trades))
    elif column == "amount_asset":
        return list(map(lambda x: x.amount_asset, csv_data.trades))
    elif column == "asset":
        return list(map(lambda x: x.asset, csv_data.trades))
    elif column == "asset_market_price":
        return list(map(lambda x: x.asset_market_price, csv_data.trades))
    elif column == "asset_market_price_currency":
        return list(map(lambda x: x.asset_market_price_currency, csv_data.trades))
    elif column == "asset_class":
        return list(map(lambda x: x.asset_class, csv_data.trades))
    elif column == "product_id":
        return list(map(lambda x: x.product_id, csv_data.trades))
    elif column == "fee":
        return list(map(lambda x: x.fee, csv_data.trades))
    elif column == "fee_asset":
        return list(map(lambda x: x.fee_asset, csv_data.trades))
    elif column == "spread":
        return list(map(lambda x: x.spread, csv_data.trades))
    elif column == "spread_currency":
        return list(map(lambda x: x.spread_currency, csv_data.trades))
    else:
        raise Exception(f"Unknown column {column}")


def get_col_by_name(trade: Trade, column: str) -> str:
    if column == "transaction_id":
        return trade.transaction_id
    elif column == "timestamp":
        return trade.timestamp
    elif column == "transaction_type":
        return trade.transaction_type
    elif column == "in_out":
        return trade.in_out
    elif column == "amount_fiat":
        return trade.amount_fiat
    elif column == "fiat":
        return trade.fiat
    elif column == "amount_asset":
        return trade.amount_asset
    elif column == "asset":
        return trade.asset
    elif column == "asset_market_price":
        return trade.asset_market_price
    elif column == "asset_market_price_currency":
        return trade.asset_market_price_currency
    elif column == "asset_class":
        return trade.asset_class
    elif column == "product_id":
        return trade.product_id
    elif column == "fee":
        return trade.fee
    elif column == "fee_asset":
        return trade.fee_asset
    elif column == "spread":
        return trade.spread
    elif column == "spread_currency":
        return trade.spread_currency
    else:
        raise Exception(f"Unknown column {column}")


def filter_rows(trades: List[Trade], column: str, value: str) -> List[CsvData]:
    return list(filter(lambda x: get_col_by_name(x, column) == value, trades))


def collect_columns(csv_data: CsvData, column: str, unique: bool) -> List[str]:
    cols = get_column(csv_data, column)
    if unique:
        cols = list(dict.fromkeys(cols))
    return cols


def parse_csv(csv_file: str) -> CsvData:
    return CsvData(csv_file)


def collect_filter_values(extra_args: List[str]) -> List[Tuple[str, str]]:
    if len(extra_args) % 2 != 0:
        raise Exception("missing argument")
    return list(zip(extra_args[0::2], extra_args[1::2]))


def main(args: List[str]) -> int:
    if len(args) < 2 or args[0] == "--help":
        print(USAGE)
        return 255

    try:
        csv_data = CsvData(args[0])
    except Exception as e:
        print(f"failed to parse csv file: {e}")
        return 1
    command = args[1]

    if command == CMD_GET_COLUMN and len(args) > 2:
        try:
            unique = len(args) == 4
            cols = collect_columns(csv_data, args[2], unique)
            for col in cols:
                print(col)
        except Exception as e:
            print(e)
            return 1
    elif command == CMD_FILTER and len(args) > 3:
        try:
            filters = collect_filter_values(args[2:])
            trades = csv_data.trades
            for (column, value) in filters:

                trades = filter_rows(trades, column, value)
            for trade in trades:
                print(trade)
        except Exception as e:
            print(e)
            return 1
    else:
        print(USAGE)
        return 255

    return 0


if __name__ == "__main__":
    main(argv[1:])

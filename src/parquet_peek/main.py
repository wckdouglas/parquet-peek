#!/usr/bin/env python
import polars as pl
import rich_click as click


@click.command(help="A tool to sneak peek parquet files")
@click.option(
    "-p", "--parquet-fn", help="Parquet file to peek", type=str, required=True
)
@click.option(
    "-l", "--lines-to-show", help="how many lines to show", type=int, default=10
)
@click.option(
    "-w",
    "--width",
    help="char width to print in each " "columns of the data frame",
    type=int,
    default=250,
)
def main(parquet_fn, lines_to_show, width):
    df = pl.scan_parquet(parquet_fn)
    with pl.Config(fmt_str_lengths=width, tbl_width_chars=width, tbl_cols=-1):
        print(df.head(lines_to_show).collect())


if __name__ == "__main__":
    main()

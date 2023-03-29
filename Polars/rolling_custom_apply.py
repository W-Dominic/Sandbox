import polars as pl

def foo(s_list: list[pl.Series]) -> pl.Series:
    x = s_list[0]
    y = s_list[1]
    res = x.sum() + y.sum()
    return pl.Series("res", [res])

if __name__ == "__main__":
    df = pl.DataFrame(
        {
        "time":[1,2,3,4,5],
        "x":[0,1,2,3,4],
        "y":[5,6,7,8,9]
        }
    )
    out = df.groupby_rolling(index_column="time", period="3i").agg(
        [
            pl.apply(exprs=["x","y"], function=foo)
            .alias("result")
        ]        
    )
    res = df.join(out, on="time", how="inner")
    print(res)

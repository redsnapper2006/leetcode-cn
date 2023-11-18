import pandas as pd

def fillMissingValues(products: pd.DataFrame) -> pd.DataFrame:
    products.loc[products['quantity'].isnull(), 'quantity'] = 0
    return products

import time

t = time.perf_counter()
import numpy as np
import pandas as pd
import sklearn
took = time.perf_counter() - t

df = pd.DataFrame({"x": np.arange(6), "y": np.arange(6) ** 2})
print(f"Hello from {{name}}!")
print(f"numpy {np.__version__}, pandas {pd.__version__}, scikit-learn {sklearn.__version__} "
      f"imported in {took * 1000:.1f} ms (already loaded in the warm snapshot)")
print(df.describe().loc[["mean", "std", "max"]])

"""Data science example — numpy, pandas, scipy, sklearn."""
import numpy as np
import pandas as pd
from scipy import stats
from sklearn.linear_model import LinearRegression

# Generate sample data
np.random.seed(42)
x = np.linspace(0, 10, 50)
y = 2.5 * x + np.random.normal(0, 2, 50)

# Fit linear regression
model = LinearRegression()
model.fit(x.reshape(-1, 1), y)

# Stats
slope, intercept, r_value, p_value, _ = stats.linregress(x, y)

# Build a DataFrame with results
df = pd.DataFrame({
    "metric": ["slope", "intercept", "r_squared", "p_value"],
    "value": [slope, intercept, r_value**2, p_value],
})

print(df.to_string(index=False))
print(f"\nsklearn coef={model.coef_[0]:.4f} intercept={model.intercept_:.4f}")

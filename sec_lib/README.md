# SEC Lib

Rust library for accessing SEC public data APIs.

## What is XBRL?
The e**X**tensible**B**usiness**R**ules**L**anguage is a scheme for labelling data in financial documents with common machine-readable labels.

A **concept** is a label applied to a specific piece of data, like "AccountsPayable" or "AdvertisingExpense". Most concepts come from the US-GAAP (Generally Accepted Accounting Principles) taxonomy.

An XBRL **fact** is a a 3-tuple of (concept, time, value).

## Interesting Concepts
[MFAANG_Concepts.json](resources/MFAANG_Concepts.json) contains a union of concepts reported by Microsoft, Facebook, Amazon, Apple and Google (Alphabet).

The GAAP taxonomy is large, and not all filings contain all concepts. Below is a listing of some common and potentially interesting concepts.

> AccountsPayable 
>
> AccountsPayableCurrent
> 
> AccountsReceivableNetCurrent
> 
> CapitalLeaseObligations - Unused post-COVID office space?
> 
> CommonStockSharesOutstanding
> 
> CostOfGoodsAndServicesSold
> 
> CostsAndExpenses
> 
> CurrentFederalTaxExpenseBenefit
> 
> CurrentForeignTaxExpenseBenefit
> 
> CurrentIncomeTaxExpenseBenefit
> 
> CurrentStateAndLocalTaxExpenseBenefit
> 
> DeferredRevenue: Payments for goods or services which the company will deliver at a later time.
> 
> Depreciation: Hand-wavy accounting magic to indicate the loss in value of assets the company owns over time.
> 
> EarningsPerShareBasic: (net income - preferred dividends) / outstanding shares
> 
> EmployeeRelatedLiabilitiesCurrent: Salaries, bonuses, payroll taxes, fringe benefits.
>
> EntityPublicFloat: Market value of voting and non-voting shares, computed from average of bid-ask spread.
>
> Goodwill: Terrible name. When purchasing another company "goodwill" is the amount payed above and beyond the estimated value of the assets acquired. May reflect value from branding, customer base, etc.
> 
> GrossProfit: Difference between revenue and cost of goods sold.
> 
> IncomeTaxesPaid: Cash paid to governments or agencies as tax on income.
> 
> IncomeTaxesPaidNet: IncomeTaxesPaid adjusted for any fine/refund for under/over payment.
> 
> InterestExpense: Expense due to interest from bonds, loans, lines of credit, convertible debt.
> 
> InventoryNet
> 
> InterestPaid
> 
> LeaseAndRentalExpense
> 
> LeaseCost
> 
> LongTermDebt
> 
> MarketingExpense
> 
> OperatingExpense
> 
> PreferredStockSharesOutstanding
> 
> PreferredStockValue
> 
> ShareBasedCompensation
> 
> StockholdersEquity
> 
> WeightedAverageNumberOfDilutedSharesOutstanding
> 
> WeightedAverageNumberOfSharesOutstandingBasic

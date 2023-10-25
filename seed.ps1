$URL = "http://192.168.1.254:5000"

$User = [PSCustomObject]@{
    username = "justinleahy"
    first_name = "Justin"
    last_name = "Leahy"
    sales_tax = 0.0875
}

$Post_User = Invoke-WebRequest -Uri "$URL/api/user" -Method POST -ContentType "application/json" -Body ($User | ConvertTo-Json)

$User_ID = ($Post_User.Content | ConvertFrom-Json).id

$Accounts = @(
    [PSCustomObject]@{
        user_id = $User_ID
        account_type = "checking"
        nickname = "Ally Checking"
        interest = 0.0009995
        interest_frequency = 1
        interest_frequency_unit = "day"
    },
    [PSCustomObject]@{
        user_id = $User_ID
        account_type = "saving"
        nickname = "Ally Saving"
        interest = 0.0416240
        interest_frequency = 1
        interest_frequency_unit = "day"
    }
)

$Post_Accounts = @()

foreach($Account in $Accounts) {
    $Post_Accounts += Invoke-WebRequest -Uri "$URL/api/account" -Method POST -ContentType "application/json" -Body ($Account | ConvertTo-Json)
}

$Checking_Account_ID = ($Post_Accounts[0].Content | ConvertFrom-Json).id
$Saving_Account_ID = ($Post_Accounts[1].Content | ConvertFrom-Json).id

$Transactions = @(
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-02-23"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 100.00
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-02-23"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 28.19
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-02-28"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 230.33
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-01"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 48.73
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-15"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 592.75
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-17"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -1000.00
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-17"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 100.00
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-18"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 125.00
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 0.85
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-03-25"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -225.85
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-04-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 0.13
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-05-03"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -0.13
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-06-30"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 150.00
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-06-30"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 462.84
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-07-14"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 680.26
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-07-14"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 88.50
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-07-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 1.67
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-07-19"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -1143.10
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-07-31"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 1108.73
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-08-05"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -435.00
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-08-15"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 1118.76
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-08-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 3.04
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-08-19"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -1732.38
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-08-31"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 1059.18
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-09-06"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -1362.50
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-09-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 1.35
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-09-30"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 1066.85
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-05"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = -1063.38
        title = "Transfer to Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-05"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 50.00
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-06"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 899.66
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-16"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 127.12
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-18"
        transaction_type = "deposit"
        transaction_category = "interest"
        amount = 2.08
        title = "Interest Paid"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-19"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 0.04
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    },
    [PSCustomObject]@{
        account = $Saving_Account_ID
        transaction_date = "2023-10-19"
        transaction_type = "transfer"
        transaction_category = "transfer"
        amount = 0.04
        title = "Transfer from Ally Checking"
        vendor = "Ally"
    }
)

$Post_Transactions = @()

foreach($Transaction in $Transactions) {
    $Post_Transactions += Invoke-WebRequest -Uri "$URL/api/account/$($Transaction.account)/transaction" -Method POST -ContentType "application/json" -Body ($Transaction | ConvertTo-Json)
}
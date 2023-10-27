const account = document.getElementById("account");

account.addEventListener("submit", (e) => {
    e.preventDefault();

    document.getElementById('result').textContent = 'User ID: ' + account.elements["users"].value;

    const data = {
        "user_id" : account.elements["users"].value,
        "account_type" : account.elements["type"].value.toLowerCase(),
        "nickname" : account.elements["nickname"].value,
        "interest" : account.elements["interest"].value,
        "interest_frequency" : account.elements["interest_frequency"].value,
        "interest_frequency_unit" : account.elements["interest_frequency_unit"].value.toLowerCase()
    };

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type' : 'application/json'
        },
        body: JSON.stringify(data)
    };

    const url = "http://localhost:5000/api/account";

    fetch(url, requestOptions)
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            return response.json();
        })
        .then(responseData => {
            document.getElementById('result').textContent = 'Response data: ' + JSON.stringify(responseData);
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
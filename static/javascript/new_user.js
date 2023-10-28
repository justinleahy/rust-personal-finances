const account = document.getElementById("user");

account.addEventListener("submit", (e) => {
    e.preventDefault();

    const data = {
        "username" : account.elements["username"].value,
        "first_name" : account.elements["first_name"].value,
        "last_name" : account.elements["last_name"].value,
        "sales_tax" : account.elements["sales_tax"].value
    };

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type' : 'application/json'
        },
        body: JSON.stringify(data)
    };

    const url = "http://" + host + ":" + port + "/api/user";

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
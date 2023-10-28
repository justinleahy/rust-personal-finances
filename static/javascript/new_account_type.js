const account_type = document.getElementById("account_type");

account_type.addEventListener("submit", (e) => {
    e.preventDefault();

    const data = {
        "label" : account_type.elements["label"].value
    };

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type' : 'application/json'
        },
        body: JSON.stringify(data)
    };

    const url = "http://" + host + ":" + port + "/api/account/type";

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
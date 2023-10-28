const transaction_category = document.getElementById("transaction_category");

transaction_category.addEventListener("submit", (e) => {
    e.preventDefault();

    const data = {
        "label" : transaction_category.elements["label"].value
    };

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type' : 'application/json'
        },
        body: JSON.stringify(data)
    };

    const url = "http://" + host + ":" + port + "/api/transaction/category";

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
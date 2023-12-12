const postData = {
    expo_push_token: "ExponentPushToken[b5nR6zALafV431QtOC7byo]",
    title: "Your Title",
    body: "Your Message Body",
};
const button = document.getElementById('push');
button.addEventListener('click', () => {
    fetch('http://127.0.0.1:3000/submit', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(postData),
    })
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            return response.json();
        })
        .then(data => {
            console.log('Success:', data);
        })
        .catch(error => {
            console.error('Error:', error);
        });
});
import requests
import time

# JSON object to be sent
data = {
    "temperature": 23.0,
    "pressure": 44.5,
    "humidity": 73.1  # Placeholder for a dynamic timestamp
}

# URL for the POST request
url = "http://localhost:5000"

def send_json_recursively(url, data, interval=1):
    """
    Sends JSON data to the specified URL recursively.

    :param url: URL to send the POST request to
    :param data: JSON data to send
    :param interval: Time interval between recursive calls in seconds
    """
    while True:
        # Add the current timestamp to the JSON data

        try:
            # Send the POST request
            response = requests.post(url, json=data)

            # Print the response
            print(f"Sent: {data}")
            print(f"Response: {response.status_code}, {response.text}")

        except requests.RequestException as e:
            print(f"Error sending data: {e}")

        # Wait for the specified interval before sending again
        time.sleep(interval)

# Start recursive sending
send_json_recursively(url, data)

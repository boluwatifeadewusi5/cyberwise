import os
import requests
from dotenv import load_dotenv

load_dotenv()

SECRET_KEY = os.getenv("SQUAD_SECRET_KEY")
PUBLIC_KEY = os.getenv("SQUAD_PUBLIC_KEY")


def initiate_transaction():
    url = "https://sandbox-api-d.squadco.com/transaction/initiate"

    headers = {
        "Authorization": SECRET_KEY,
        "Content-Type": "application/json"
    }

    payload = {
        "email": "semajayi1234@gmail.com",
        "currency": "NGN",
        "initiate_type": "inline",
        "callback_url": "https://www.linkedin.com/",
        "amount": 20000
    }

    response = requests.post(
        url,
        headers=headers,
        json=payload
    )

    print(response.text)

    attempt_transaction()


def attempt_transaction():
    url = "https://sandbox-api-d.squadco.com/virtual-account/simulate/payment"

    headers = {
        "Authorization": SECRET_KEY,
        "Content-Type": "application/json"
    }

    payload = {
        "virtual_account_number": "9279755518",
        "amount": 20000
    }

    response = requests.post(
        url,
        headers=headers,
        json=payload
    )

    print("SIMULATION RESPONSE:")
    print(response.text)


initiate_transaction()